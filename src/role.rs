use crate::state::TgptState;

//TODO cleanup role builder
pub fn role_builder(state: &TgptState) -> String {
    state.config.role[0].prompt.clone()

    //String::from("IMPORTANT: if asked a technical question only only provide code as output without any description. Provide only code in plain text format without Markdown formatting. Do not include symbols such as ``` or ```python")
}
