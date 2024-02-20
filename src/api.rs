use leptos::*;
use cfg_if::cfg_if;
use crate::model::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(prompt: Conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    let model = extract(|data: actix_web::web::Data<Llama>| {
        data.into_inner()
    })
        .await?;

    use llm::KnownModel;
    let mut session = (*model).start_session(Default::default());
    let inference_parameters = llm::InferenceParameters::default();
    let character_name = "### Assistant";
    let user_name = "### Human";
    let persona = "A chat between a human and an assistant.";
    let mut history = format!(
        "{character_name}: Hello - How may I help you today?\n\
        {user_name}: What is the capital of France?\n\
        {character_name}:  Paris is the capital of France.\n"
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let curr_line = if message.user {
            format!("{character_name}: {msg}\n")
        } else {
            format!("{user_name}: {msg}\n")
        };

        history.push_str(&curr_line);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();

    // Build the format for how the AI should infer a response
    session
        .infer(
            model.as_ref(),
            &mut rng,
            &llm::InferenceRequest {
                prompt: format!("{persona}\n{history}\n{character_name}:")
                    .as_str()
                    .into(),
                parameters: &inference_parameters,
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            inference_callback(String::from(user_name), &mut buf, &mut res),
        )
        .unwrap_or_else(|e| panic!("{e}"));
    Ok(res)
}

// Compilation for an inference callback function when ssr is active
cfg_if! {
    if #[cfg(feature = "ssr")] {
    use std::convert::Infallible;
        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            out_str: &'a mut String,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, Infallible> + 'a {

            use llm::InferenceFeedback::Halt;
            use llm::InferenceFeedback::Continue;

            // Closure to handle inference responses, including inferred tokens and end-of-token (EOT) signals
            move |resp| match resp {
                llm::InferenceResponse::InferredToken(t) => {
                    let mut reverse_buf = buf.clone();
                    reverse_buf.push_str(t.as_str());
                    if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                        buf.clear();
                        return Ok::<llm::InferenceFeedback, Infallible>(Halt);
                    } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                        buf.push_str(t.as_str());
                        return Ok(Continue);
                    }

                    if buf.is_empty() {
                        out_str.push_str(&t);
                    } else {
                        out_str.push_str(&reverse_buf);
                    }

                    Ok(Continue)
                }
                llm::InferenceResponse::EotToken => Ok(Halt),
                _ => Ok(Continue),
            }
        }
    }
}
// Extracting data from a request with ssr
#[cfg(feature = "ssr")]
pub async fn extract<F, E, T>(f: F) -> Result<T, ServerFnError>
    where
        F: FnOnce(E) -> T,
        E: actix_web::FromRequest,
        <E as actix_web::FromRequest>::Error: std::fmt::Display,
{
    let req = use_context::<actix_web::HttpRequest>()
        .expect("HttpRequest should have been provided via context");
    let input = E::extract(&req)
        .await
        .map_err(|e| ServerFnError::ServerError(e.to_string()))?;
    Ok(f(input))
}