use gtk4::prelude::*;
use gtk4::{glib, prelude::ApplicationExt};
use glib::clone;
use gtk4::{Application, ApplicationWindow, Button, Box, Align, Orientation, Label, Entry};
use eval::eval;

fn main() {
    let app = Application::builder()
        .application_id("rusty.calc")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application)
{
    let container = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(24)
        .margin_end(24)
        .margin_start(24)
        .margin_bottom(24)
        .halign(Align::Center)
        .valign(Align::Center)
        .spacing(24)
        .build();
    
            
    let io_container = Box::builder()
        .orientation(Orientation::Vertical)
        .halign(Align::Center)
        .spacing(24)
        .build();
    
    let input = Entry::builder()
        .placeholder_text("Enter input")
        .build();
        
    let result = "";
        
    let output = Label::builder()
        .label(&format!("Result: {}", result))
        .build();
        
    io_container.append(&input);
    io_container.append(&output);
    container.append(&io_container);
    
    let operation_container_r1 = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .spacing(24)
        .build();
        
    let operation_container_r2 = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .spacing(24)
        .build();
    
    let operation_container_r3 = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .spacing(24)
        .build();
        
    let operation_container_r4 = Box::builder()
        .orientation(Orientation::Horizontal)
        .halign(Align::Center)
        .spacing(24)
        .build();
    
    let add_button = Button::builder()
        .label("+")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let sub_button = Button::builder()
        .label("-")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let mult_button = Button::builder()
        .label("*")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let div_button = Button::builder()
        .label("/")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let equal_button = Button::builder()
        .label("=")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    let one_button = Button::builder()
        .label("1")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let two_button = Button::builder()
        .label("2")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let three_button = Button::builder()
        .label("3")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let four_button = Button::builder()
        .label("4")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let five_button = Button::builder()
        .label("5")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let six_button = Button::builder()
        .label("6")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let seven_button = Button::builder()
        .label("7")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let eight_button = Button::builder()
        .label("8")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let nine_button = Button::builder()
        .label("9")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    let zero_button = Button::builder()
        .label("0")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    let dot_button = Button::builder()
        .label(".")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let pls_min_button = Button::builder()
        .label("+/-")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let clear_button = Button::builder()
        .label("clr")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let openpar_button = Button::builder()
        .label("(")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
        
    let closepar_button = Button::builder()
        .label(")")
        .margin_top(12)
        .margin_start(12)
        .margin_end(12)
        .build();
           
     
    one_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("1");
        input.set_text(&old_input_text);
    }));
    
    two_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("2");
        input.set_text(&old_input_text);
    }));
    
    three_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("3");
        input.set_text(&old_input_text);
    }));
        
    four_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("4");
        input.set_text(&old_input_text);
    }));
        
    five_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("5");
        input.set_text(&old_input_text);
    }));
        
    six_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("6");
        input.set_text(&old_input_text);
    }));
        
    seven_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("7");
        input.set_text(&old_input_text);
    }));
        
    eight_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("8");
        input.set_text(&old_input_text);
    }));
        
    nine_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("9");
        input.set_text(&old_input_text);
    }));
        
    zero_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("0");
        input.set_text(&old_input_text);
    }));
        
    dot_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str(".");
        input.set_text(&old_input_text);
    }));
        
    openpar_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("(");
        input.set_text(&old_input_text);
    }));
        
    closepar_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str(")");
        input.set_text(&old_input_text);
    }));
        
    add_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("+");
        input.set_text(&old_input_text);
    }));
        
    sub_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("-");
        input.set_text(&old_input_text);
    }));
        
    mult_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("*");
        input.set_text(&old_input_text);
    }));
        
    div_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("/");
        input.set_text(&old_input_text);
    }));
        
    pls_min_button.connect_clicked(clone!(@weak input =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.push_str("*(0-1)");
        input.set_text(&old_input_text);
    }));
        
    clear_button.connect_clicked(clone!(@weak input , @weak output =>
        move |_| {
        let mut old_input_text = input.text().to_string();
        old_input_text.clear();
        input.set_text(&old_input_text);
        output.set_text(&format!("Result: "));
    }));
        
    equal_button.connect_clicked(clone!(@weak input, @weak output =>
        move |_| {
            let entree = input.text().to_string();  
            match eval(&entree)
            {
                Ok(result) => 
                {
                    output.set_text(&format!("Result: {}", &result));        
                }
                
                Err(e) =>
                {
                    println!("{}",e);
                    output.set_text("Result: ERROR");
                }
            }
        }));
        
    operation_container_r1.append(&one_button);
    operation_container_r1.append(&two_button);
    operation_container_r1.append(&three_button);
    operation_container_r1.append(&add_button);
    operation_container_r1.append(&sub_button);
    
    operation_container_r2.append(&four_button);
    operation_container_r2.append(&five_button);
    operation_container_r2.append(&six_button);
    operation_container_r2.append(&mult_button);
    operation_container_r2.append(&div_button);
    
    operation_container_r3.append(&seven_button);
    operation_container_r3.append(&eight_button);
    operation_container_r3.append(&nine_button);
    operation_container_r3.append(&openpar_button);
    operation_container_r3.append(&closepar_button);
    
    operation_container_r4.append(&zero_button);
    operation_container_r4.append(&dot_button);
    operation_container_r4.append(&pls_min_button);
    operation_container_r4.append(&equal_button);
    operation_container_r4.append(&clear_button);
        
    container.append(&operation_container_r1);
    container.append(&operation_container_r2);
    container.append(&operation_container_r3);
    container.append(&operation_container_r4);
        
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(300)
        .default_height(600)
        .title("Calculatrice")
        .child(&container)
        .build();
        
    window.show();
}
