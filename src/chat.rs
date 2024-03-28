use async_openai::types::{
    ChatChoice, ChatCompletionRequestMessage, ChatCompletionRequestUserMessageArgs, CreateChatCompletionResponse
};

use crate::Result;

pub fn user_msg(content: impl Into<String>) -> Result<ChatCompletionRequestMessage> {
    let msg = ChatCompletionRequestUserMessageArgs::default()
        .content(content.into())
        .build()?;
    Ok(msg.into())
}

pub fn first_choice(chat_response: CreateChatCompletionResponse) -> Result<ChatChoice> {
    let first_choice = chat_response
        .choices
        .into_iter()
        .next()
        .ok_or("No first choice?")?;

    Ok(first_choice)
}
