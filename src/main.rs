use std::ops::Add;
use std::path::Iter;

trait NodeTrait {}


struct Leaf<T: Patient> {
    name: String,
    patient: Box<T>
}

impl<T: Patient> NodeTrait for Leaf<T> {}

impl<T: Patient, Child: NodeTrait> Leaf<T> {
    fn add_child(self, child: Child) -> Twig<T, Child> {
        Twig {
            name: self.name,
            patient: self.patient,
            child: child
        }
    }
}


struct Twig<T: Patient, Child: NodeTrait> {
    name: String,
    patient: Box<T>,
    child: Child
}

impl<T: Patient, Child: NodeTrait> NodeTrait for Twig<T, Child> {}

impl<T: Patient, Child: NodeTrait> Twig<T, Child> {
    fn delete_child(self) -> Leaf<T> {
        Leaf {
            name: self.name,
            patient: self.patient
        }
    }
}


struct Branch<T: Patient, Left: NodeTrait, Right: NodeTrait> {
    name: String,
    patient: Box<T>,
    left: Left,
    right: Right
}

impl<T: Patient, Left: NodeTrait, Right: NodeTrait> NodeTrait for Branch<T, Left, Right> {}


struct Node {
    name: std::string::String,
    patient: Box<dyn Patient>,
    children: Vec<Node>
}

impl Node {
    fn update(&mut self) -> Option<NodeMessage> {
        let children_messages = self.children
            .iter_mut()
            .map(|child| child.update())
            .collect::<Vec<Option<NodeMessage>>>();

        for message in children_messages {
            if let Some(real_message) = message {
                if let NodeMessage::Parent(message_contents) = real_message {
                    message_contents(self);
                }
            }
        }

        let patient_message = self.patient.update();

        if let Some(real_message) = patient_message {
            if let NodeMessage::Own(message_contents) = real_message {
                message_contents(self);
            }

            if let NodeMessage::Parent(message_contents) = real_message {
                return Some(NodeMessage::Parent(message_contents));
            }
        }

        return None;
    }
}

impl Printable for Node {
    fn print(&self) -> String {
        return self.name.clone(); // hmm thinking emoji
    }
}


impl NodeMessage {
    fn new() -> NodeMessage {
        return NodeMessage::Own(|_| {});
    }
}

enum NodeMessage {
    Own(fn(&mut Node)),
    Parent(fn(&mut Node)),
    Deep(fn(&mut Node))
}


trait Patient {
    fn update(&mut self) -> Option<NodeMessage>;
}


struct Actor {
    position: i8
}

impl Patient for Actor {
    fn update(&mut self) -> Option<NodeMessage> {
        self.position += 1;

        return None;
    }
}


trait Printable {
    fn print(&self) -> String;
}

struct Printer {}

impl Printer {
    fn print(node: &Node, depth: usize) {
        let spaces = String::from(" ".repeat(depth));
        println!("{}{}", spaces, node.name);
    }

    fn print_recursive(node: &Node, depth: usize) {
        Printer::print(node, depth);
        for child in node.children.iter() {
            Printer::print_recursive(child, depth + 1);
        }
    }
}

impl Patient for Printer {
    fn update(&mut self) -> Option<NodeMessage> {
        return Some(NodeMessage::Parent(|parent| { Printer::print_recursive(parent, 0) }));
    }
}


fn main() {
    let printer = Box::new(Printer {});
    let printer_node = Node { name: String::from("printer"), patient: printer, children: Vec::new() };

    let parent: Box<Actor> = Box::new(Actor { position: 0 });
    let mut parent_node: Node = Node { name: String::from("parent"), patient: parent, children: vec![printer_node] };

    for _ in 0..5 {
        let _ = parent_node.update();
    }
}
