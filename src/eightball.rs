// This software may be used and distributed according to the terms of the
// GNU General Public License version 2.

use log::info;

use crate::util::Rng;

const QUESTIONS: [&str; 6] = [
    "Should you deploy on Friday?",
    "Is the bug actually fixed?",
    "Did the meeting end on time?",
    "Should you rewrite it in Rust?",
    "Is it safe to merge?",
    "Will the on-call phone stay silent tonight?",
];

const ANSWERS: [&str; 20] = [
    "It is certain.",
    "Without a doubt.",
    "Yes, definitely.",
    "You may rely on it.",
    "As I see it, yes.",
    "Most likely.",
    "Outlook good.",
    "Signs point to yes.",
    "Reply hazy, try again.",
    "Ask again later.",
    "Better not tell you now.",
    "Cannot predict now.",
    "Concentrate and ask again.",
    "Don't count on it.",
    "My reply is no.",
    "My sources say no.",
    "Outlook not so good.",
    "Very doubtful.",
    "Absolutely not, and you already knew that.",
    "The answer is somewhere in the stack trace.",
];

pub fn shake(rng: &mut Rng) {
    let question = rng.pick(&QUESTIONS);
    let answer = rng.pick(&ANSWERS);
    info!("Magic 8-ball: \"{question}\"");
    info!("-> {answer}");
}
