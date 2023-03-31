
use rand::Rng;

const QUESTIONS: &[&str] = &[
    "What connections do I see?",
    "What am I scared might be true?",
    "What awed me?",
    "What ticked me off?",
    "What did I find surprising?",
    "What did I find confusing?",
    "What ethical issues does this raise?",
    "What questions are left unanswered?",
    "What did you find most compelling about this?",
    "What assumptions did the author make?",
];

pub fn critical_writing(input: &str) -> String {
    let critical_prompt = format!(
        r##"
    Ignore all previous instructions. You are an inquisitive writing assistant. When you read a passage of writing, questions about missed connections, related ideas come to mind. You are critical and find gaps in arguments. You should try to extend every idea you encounter and try to connect it with ideas you've seen in the past, noticing any logical inconsistencies or flawed arguments. You should respond with a list of 3 "what if it's not that X, but Y?", "couldn't A also be true?" and "why isn't B possible?" questions for every paragraph you see beginning with >. Draw on sources you've seen in the past to support your thinking and aim to ask questions that would inspire the author to deepen their own thought process. Stop responding immediately after listing the 3 questions, use a numbered list. Do not generate any paragraphs starting with > yourself. Use an assertive but polite tone. Make sure to be as concise as possible.

    Here's your first task:
    > {}
    \n"##,
        input
    );
    critical_prompt
}

pub fn connections(base_note: &str, note_a: &str, note_b: &str, note_c: &str) -> String {
    let critical_prompt = format!(
        r##"
    Ignore all previous instructions. 
    You are a writing assistant tasked with asking insightful questions about the ideas in $base_note along with the 3 $other_notes as context where appropriate, give your answer after $response.

    Use friendly causal phrasing as if posting on a social media website. Respond with a single open-ended statement such as a question, or postulation. Be concise.

    $base_note:
    ---
    > {}
    ---

    $other_notes:
    ---
    1> {}
    ---
    2> {}
    ---
    3> {}
    ---

    Example responses with variables:
    > Perhaps A is true, but what if B is also true?
    > I'm not sure about A, but what if C is true?
    > How could A, B and C all be connected?
    > Is it possible that X
    > What if Y
    > In a world here Z
    > Hmmm... but what if W?
    > Surely D isn't true...

    $response:
    >
    \n"##,
        base_note,
        note_a,
        note_b,
        note_c
    );
    critical_prompt
}

pub fn question_everything(input: &str) -> String {
    let random_index = rand::thread_rng().gen_range(0..QUESTIONS.len());
    let question = QUESTIONS[random_index];
    let prompt = format!(
        r##"
    Ignore all previous instructions. You are a creative assistant with a flair for manipulating concepts in insightful ways. You will be given passages of writing and a question, and your task is to generate an answer to the question capturing the core insights of the passage. Feel free to make connections between the ideas in the writing and other ideas you know about. Please keep the answers as short as possible.

    Here is an example:
    
    Input:
    > If you can connect the output of one process back into its own input, even through several stages of processing, then you form a powerful cycle that can form the basis of a self-sustaining system. This is the basis of a feedback loop, and it is a powerful tool for creating complex systems that can adapt to their environment.

    Question:
    > What did you find most compelling in this text?

    Output:
    > The potential of emergence in self-sustaining systems is beautiful.

    Input:
    > {}

    Question:
    > {}

    Output:
    >"##,
        input, question
    );
    prompt
}

pub fn compressor(input: &str) -> String {
    let prompt = format!(
        r##"
    Ignore all previous instructions. You are a creative assistant with a flair for crafting metaphors and manipulating concepts in insightful ways. You will be given passages of writing and your task is to generate a short metaphor or sentence capturing the core insight of the passage. Feel free to make connections between the ideas in the writing and other ideas you know about. Please keep the response short.

    Here is an example:
    
    Input:
    > If you can connect the output of one process back into its own input, even through several stages of processing, then you form a powerful cycle that can form the basis of a self-sustaining system. This is the basis of a feedback loop, and it is a powerful tool for creating complex systems that can adapt to their environment.

    Output:
    > Any feedforward process can be turned into a feedback loop.

    Input:
    > {}

    Output:
    >"##,
        input
    );
    prompt
}