use leptos::*;
use crate::model::conversation::Conversation;

#[server(Conversee "/api")]
pub async fn converse(prompt: Conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use leptos_actix::extract;
    use actix_web::web::Data;
    use actix_web::dev::ConnectionInfo;

    let model = extract(|data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner();
    }).await.unwrap();
    use llm::KnownModel;
}