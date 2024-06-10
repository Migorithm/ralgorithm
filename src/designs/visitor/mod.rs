struct EventA;
struct EventB;
struct EventC;

trait TSender {
    fn send_event_a(&self, e: &EventA);
    fn send_event_b(&self, o: &EventB);
    fn send_event_c(&self, o: &EventC);
}

trait TNotification {
    fn accept(&self, sender: &dyn TSender);
}

impl TNotification for EventA {
    fn accept(&self, sender: &dyn TSender) {
        sender.send_event_a(self);
    }
}
impl TNotification for EventB {
    fn accept(&self, sender: &dyn TSender) {
        sender.send_event_b(self);
    }
}

impl TNotification for EventC {
    fn accept(&self, sender: &dyn TSender) {
        sender.send_event_c(self);
    }
}

// Concrete visitor
struct EmailSender;

impl TSender for EmailSender {
    fn send_event_a(&self, e: &EventA) {
        println!("Sending email for EventA");
    }

    fn send_event_b(&self, e: &EventB) {
        println!("Sending email for EventB");
    }

    fn send_event_c(&self, e: &EventC) {
        println!("Sending email for EventC");
    }
}

struct SlackSender;
impl TSender for SlackSender {
    fn send_event_a(&self, e: &EventA) {
        println!("Sending slack message for EventA");
    }

    fn send_event_b(&self, e: &EventB) {
        println!("Sending slack message for EventB");
    }

    fn send_event_c(&self, e: &EventC) {
        println!("Sending slack message for EventC");
    }
}

fn main() {
    let events: Vec<Box<dyn TNotification>> =
        vec![Box::new(EventA), Box::new(EventB), Box::new(EventC)];

    let email_visitor = EmailSender;
    let slack_visitor = SlackSender;

    for event in events {
        event.accept(&email_visitor);
        event.accept(&slack_visitor);
    }
}
