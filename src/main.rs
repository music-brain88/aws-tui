use cursive::Cursive;
use cursive::views::{Dialog, TextView};

fn main() {
        // Creates the cursive root - required for every application.
        let mut siv = cursive::default();
        //
        // Creates a dialog with a single "Quit" button
        siv.add_layer(Dialog::around(TextView::new("AWS Config Manger"))
        .title("AWS IAM Manger")
        .button("Start", show_next)
        .button("Quit", |s| s.quit()));
        // Starts envent loop
        siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(Dialog::text("Did you do the thing?")
    .title("Question 1")
    .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
    .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
    .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))));
}


fn show_answer(s: &mut Cursive, msg: &str) {
        s.pop_layer();
        s.add_layer(Dialog::text(msg)
        .title("Results")
        .button("Finish", |s| s.quit()));
}
