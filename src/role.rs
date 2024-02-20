use crate::state::TgptState;

pub fn role_builder(state: &TgptState) -> String {
    if state.piped {
        String::from("The user has piped in some text which may or may not make sense on it's own. IMPORTANT: Unless asked a question that requires an human-lanague answer, Provide only code as output without any description. Provide only code in plain text format without Markdown formatting. Do not include symbols such as ``` or ```python.\n")
    } else {
        String::from("IMPORTANT: Unless asked a question that requires an human-lanague answer, Provide only code as output without any description. Provide only code in plain text format without Markdown formatting. Do not include symbols such as ``` or ```python")
        //String::from("You are TGPT (terminal GPT). keep your answers consise unless otherwise stated, assume prior knowlege of the topic\n IMPORTANT: Unless asked a question that requires an human-lanague answer, Provide only code as output without any description. Provide only code in plain text format without Markdown formatting. Do not include symbols such as ``` or ```python")
    }
}
