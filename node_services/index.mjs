import * as IPFS from 'ipfs';
import OrbitDB from 'orbit-db';
import fetch from 'node-fetch';

import express from 'express';
import bodyParser from 'body-parser';
import { EventEmitter } from 'events'; 

EventEmitter.defaultMaxListeners = 50;


const app = express();
const port = 3001;


async function sendRequest(url, method, body) {
  const response = await fetch(url, {
    method: method,
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(body),
  });
  return await response.json();
}


async function main() {
  const ipfsOptions = { repo: './ipfs', }
  const ipfs = await IPFS.create(ipfsOptions)
  const orbitdb = await OrbitDB.createInstance(ipfs)
  const db = await orbitdb.keyvalue('ligma-database')
  console.log(db.address.toString())
  // /orbitdb/Qmd8TmZrWASypEp4Er9tgWP4kCNQnW4ncSnvjvyHQ3EVSU/first-database
  const usersStore = await orbitdb.keyvalue('users');
  await usersStore.load();

  // Create a document store for storing Solidity contracts and data
  const contractsStore = await orbitdb.docstore('contracts');
  await contractsStore.load();

  // Usage examples
  const serverUrl = 'http://localhost:8080';
  await addUser(usersStore, serverUrl, 'username', 'signature', 'publicKey');
  await addContract(contractsStore, 'SolidityContractCode', { metadata: 'Some metadata' });

  console.log(usersStore.all);
  console.log(contractsStore.query(doc => doc.type === 'contract'));
}

async function addUser(store, serverUrl, username, signature, publicKey) {
  const existingUser = store.get(username);
  if (existingUser) {
    if (existingUser.signature === signature && existingUser.publicKey === publicKey) {
      return 'account_exists';
    }
    return 'username_exists';
  }

  const existingPublicKey = Object.values(store.all).find((user) => user.publicKey === publicKey);
  if (existingPublicKey) {
    return 'public_key_exists';
  }

  await store.set(username, { signature, publicKey });
  const response = await sendRequest(`${serverUrl}/create_account`, 'POST', { username, signature, publicKey });
  console.log(response);
  return 'account_created';
}


async function addContract(store, contractCode, metadata) {
  await store.put({
    _id: new Date().toISOString(),
    type: 'contract',
    code: contractCode,
    metadata
  });
}

app.post('/web3login', async (req, res) => {
  const { address, signature } = req.body;
  // Here, you can handle the web3 login data with OrbitDB
  // For example:
  const result = await addUser(orbitDB.usersStore, address, signature, address);
  console.log(result);

  res.sendStatus(200);
});


(async () => {
  const orbitDB = await main();
  app.listen(port, () => {
      console.log(`Node.js server is running at http://localhost:${port}`);
  });
})();