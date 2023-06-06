use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::components::list_errors::ListErrors;
use crate::routes::AppRoute;
use crate::services::articles::*;
use crate::types::{ArticleCreateUpdateInfo, ArticleCreateUpdateInfoWrapper};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub slug: Option<String>,
}
// Create or update an article
#[function_component(Editor)]
pub fn editor(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();
    let error = use_state(|| None);
    let update_info = use_state(ArticleCreateUpdateInfo::default);

    let article_get = {
        let slug = props.slug.clone();
        use_async(async move { get(slug.unwrap_or_default()).await })
    };
    let article_update = {
        let slug = props.slug.clone();
        let update_info = update_info.clone();
        use_async(async move {
            let request = ArticleCreateUpdateInfoWrapper {
                article: (*update_info).clone(),
            };
            if let Some(slug) = slug {
                update(slug, request).await
            } else {
                create(request).await
            }
        })
    };

    {
        let article_get = article_get.clone();
        use_effect_with_deps(
            move |slug| {
                if slug.is_some() {
                    article_get.run();
                }
                || ()
            },
            props.slug.clone(),
        );
    }

    {
        let update_info = update_info.clone();
        let error = error.clone();
        use_effect_with_deps(
            move |article_get| {
                if let Some(article_info) = &article_get.data {
                    update_info.set(ArticleCreateUpdateInfo {
                        title: article_info.article.title.clone(),
                        description: article_info.article.description.clone(),
                        body: article_info.article.body.clone(),
                    });
                    error.set(None);
                }
                if let Some(e) = &article_get.error {
                    error.set(Some(e.clone()));
                }

                || ()
            },
            article_get,
        );
    }

    {
        let error = error.clone();
        use_effect_with_deps(
            move |article_update| {
                if let Some(article_info) = &article_update.data {
                    error.set(None);
                    // Route to article detail page.
                    navigator.push(&AppRoute::Article {
                        slug: article_info.article.slug.clone(),
                    });
                }
                if let Some(e) = &article_update.error {
                    error.set(Some(e.clone()));
                }
                || ()
            },
            article_update.clone(),
        );
    }

    let onsubmit = {
        let article_update = article_update.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            article_update.run();
        })
    };
    let oninput_title = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.title = input.value();
            update_info.set(info);
        })
    };
    let oninput_description = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.description = input.value();
            update_info.set(info);
        })
    };
    let oninput_body = {
        let update_info = update_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*update_info).clone();
            info.body = input.value();
            update_info.set(info);
        })
    };
  
    let onkeypress = Callback::from(|e: KeyboardEvent| {
        // Prevent submit the form when press Enter
        if e.key() == "Enter" {
            e.prevent_default();
        }
    });

    html! {
        <div class="editor-page">
            <div class="container page">
                <div class="row">
                    <div class="col-md-10 offset-md-1 col-xs-12">
                        <ListErrors error={(*error).clone()} />
                        <form {onsubmit}>
                            <fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control form-control-lg"
                                        type="text"
                                        placeholder="Article Title"
                                        value={update_info.title.clone()}
                                        oninput={oninput_title} />
                                </fieldset>
                                <fieldset class="form-group">
                                    <input
                                        class="form-control"
                                        type="text"
                                        placeholder="What's this article about?"
                                        value={update_info.description.clone()}
                                        oninput={oninput_description} />
                                </fieldset>
                                <fieldset class="form-group">
                                    <textarea
                                        class="form-control"
                                        rows="8"
                                        placeholder="Write your article (in markdown)"
                                        value={update_info.body.clone()}
                                        oninput={oninput_body} >
                                    </textarea>
                                </fieldset>
                                <fieldset class="form-group">
                                  
                                   
                                </fieldset>
                                <button
                                    class="btn btn-lg pull-xs-right btn-primary"
                                    type="submit"
                                    disabled={article_update.loading}>
                                    { "Publish Article" }
                                </button>
                            </fieldset>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    }
}
