# redesigned-eureka
- [x] Take in the address from metamask.
    - [] master the state and have it generate UserData on the frontend

- [] Create the ducking navbar.
    - [] contextnav
    - [] searchbar

- [x] fix the server
    - [x] handle login request

    - [] profile generation
        - [] display on the frontend.
        - [] convert them over to wasm.
    - [] websocket
    - [] upgrade security.
    - [x] handle create account request
        - [x] If the username or email is taken, return an error.
    - [] All usernames are stored in orbitdb including web2 accounts.
    - [x] If they use a regular login emails are stored in mongodb.
    - [x] Generate crypto addresses
        - [] ethereum
        - [] XMR
        - [] Bitcoin
        - [x] mongo will hold private keys. & emails.
- [] Proection from DDOS.
- []