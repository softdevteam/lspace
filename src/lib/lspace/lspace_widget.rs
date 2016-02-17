#![cfg_attr(not(feature = "gtk_3_10"), allow(unused_variables, unused_mut))]

extern crate time;

use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;

use gdk::ffi as gdk_ffi;
use gtk;
use gtk::traits::*;
use gtk::signal::Inhibit;

use input::inputmodifier::{self, InputModifierState};

use geom::point2::Point2;
use pres::pres::Pres;
use lspace_area::LSpaceArea;



fn gdk_modifier_to_input_mod_state(gdk_state: gdk_ffi::GdkModifierType) -> InputModifierState {
    let mut
    value = 0;
    value = value | if gdk_state.contains(gdk_ffi::GDK_BUTTON1_MASK) {inputmodifier::BUTTON1}
                    else {0};
    value = value | if gdk_state.contains(gdk_ffi::GDK_BUTTON2_MASK) {inputmodifier::BUTTON2}
                    else {0};
    value = value | if gdk_state.contains(gdk_ffi::GDK_BUTTON3_MASK) {inputmodifier::BUTTON3}
                    else {0};
    value = value | if gdk_state.contains(gdk_ffi::GDK_BUTTON4_MASK) {inputmodifier::BUTTON4}
                    else {0};
    value = value | if gdk_state.contains(gdk_ffi::GDK_BUTTON5_MASK) {inputmodifier::BUTTON5}
                    else {0};

    value = value | if gdk_state.contains(gdk_ffi::GDK_CONTROL_MASK) {inputmodifier::KEY_CTRL}
                    else {0};
    value = value | if gdk_state.contains(gdk_ffi::GDK_SHIFT_MASK) {inputmodifier::KEY_SHIFT}
                    else {0};
    value = value | if gdk_state.contains(gdk_ffi::GDK_MOD1_MASK) {inputmodifier::KEY_ALT}
                    else {0};
    value = value | if gdk_state.contains(gdk_ffi::GDK_SUPER_MASK) {inputmodifier::KEY_SUPER}
                    else {0};
    InputModifierState::from_values(value)
}

pub struct LSpaceWidget {
    drawing_area: Rc<gtk::DrawingArea>,
    state: Rc<LSpaceArea>

}

impl LSpaceWidget {
    pub fn new_with_area(area: LSpaceArea) -> Rc<RefCell<LSpaceWidget>> {
        let drawing_area = Rc::new(gtk::DrawingArea::new().unwrap());
        let wrapped_state = Rc::new(area);

        let instance = LSpaceWidget{drawing_area: drawing_area.clone(),
            state: wrapped_state.clone()
        };
        let wrapped_instance = Rc::new(RefCell::new(instance));

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_realize(move |widget| {
                state_clone.on_realize();
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_unrealize(move |widget| {
                state_clone.on_unrealize();
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_size_allocate(move |widget, rect| {
                state_clone.on_size_allocate(rect.width as i32, rect.height as i32);
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_button_press_event(move |widget, event_button| {
                let mod_state = gdk_modifier_to_input_mod_state(event_button.state);
                let pos = Point2::new(event_button.x, event_button.y);
                state_clone.on_button_press(mod_state, pos, event_button.button);
                return Inhibit(true);
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_button_release_event(move |widget, event_button| {
                let mod_state = gdk_modifier_to_input_mod_state(event_button.state);
                let pos = Point2::new(event_button.x, event_button.y);
                state_clone.on_button_release(mod_state, pos, event_button.button);
                return Inhibit(true);
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_enter_notify_event(move |widget, event_crossing| {
                let mod_state = gdk_modifier_to_input_mod_state(event_crossing.state);
                let pos = Point2::new(event_crossing.x, event_crossing.y);
                state_clone.on_enter(mod_state, pos);
                return Inhibit(true);
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_leave_notify_event(move |widget, event_crossing| {
                let mod_state = gdk_modifier_to_input_mod_state(event_crossing.state);
                let pos = Point2::new(event_crossing.x, event_crossing.y);
                state_clone.on_leave(mod_state, pos);
                return Inhibit(true);
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_motion_notify_event(move |widget, event_motion| {
                let mod_state = gdk_modifier_to_input_mod_state(event_motion.state);
                let pos = Point2::new(event_motion.x, event_motion.y);
                state_clone.on_motion(mod_state, pos);
                return Inhibit(true);
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_scroll_event(move |widget, event_scroll| {
                let mod_state = gdk_modifier_to_input_mod_state(event_scroll.state);
                let pos = Point2::new(event_scroll.x, event_scroll.y);
                state_clone.on_scroll(mod_state, pos, event_scroll.delta_x, event_scroll.delta_y);
                return Inhibit(true);
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_key_press_event(move |widget, event_key| {
                let mod_state = gdk_modifier_to_input_mod_state(event_key.state);
                let mut key_string = String::new();
                unsafe {
                    key_string.push(ptr::read(event_key.string));
                }
                state_clone.on_key_press(mod_state, event_key.keyval, key_string);
                return Inhibit(true);
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_key_release_event(move |widget, event_key| {
                let mod_state = gdk_modifier_to_input_mod_state(event_key.state);
                let mut key_string = String::new();
                unsafe {
                    key_string.push(ptr::read(event_key.string));
                }
                state_clone.on_key_release(mod_state, event_key.keyval, key_string);
                return Inhibit(true);
            });
        }

        {
            let state_clone = wrapped_state.clone();
            drawing_area.connect_draw(move |widget, cairo_context| {
                state_clone.on_draw(&cairo_context);
                return Inhibit(true);
            });
        }

        return wrapped_instance;
    }

    pub fn new(content: Pres) -> Rc<RefCell<LSpaceWidget>> {
        let area = LSpaceArea::new();
        area.set_content_pres(content);
        return LSpaceWidget::new_with_area(area);
    }

    pub fn gtk_widget(&self) -> Rc<gtk::DrawingArea> {
        return self.drawing_area.clone();
    }
}
