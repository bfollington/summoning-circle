# summoning-circle
Designing spirits using GPT-3 and friends

This is a little hacking project to design prompts to draw our certain modes from GPT-3 and then invoke them programmatically.

## Running & Development

You'll need a `.env` file with `API_PATH` (typically set to `https://api.openai.com/v1`) and `API_KEY` set to your OpenAI key.

Then `cargo run` will present you with a REPL where you can send text and receive responses.
