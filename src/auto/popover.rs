// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
#[cfg(any(feature = "v3_20", feature = "dox"))]
use PopoverConstraint;
#[cfg(any(feature = "v3_12", feature = "dox"))]
use PositionType;
use Widget;
use ffi;
#[cfg(any(feature = "v3_12", feature = "dox"))]
use gdk;
#[cfg(any(feature = "v3_12", feature = "dox"))]
use gio;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct Popover(Object<ffi::GtkPopover, ffi::GtkPopoverClass>): Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_popover_get_type(),
    }
}

impl Popover {
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    pub fn new<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(relative_to: Q) -> Popover {
        assert_initialized_main_thread!();
        let relative_to = relative_to.into();
        let relative_to = relative_to.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_new(relative_to.0)).downcast_unchecked()
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    pub fn new_from_model<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>, R: IsA<gio::MenuModel>>(relative_to: Q, model: &R) -> Popover {
        assert_initialized_main_thread!();
        let relative_to = relative_to.into();
        let relative_to = relative_to.to_glib_none();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_new_from_model(relative_to.0, model.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait PopoverExt: 'static {
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn bind_model<'a, 'b, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(&self, model: Q, action_namespace: R);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_constrain_to(&self) -> PopoverConstraint;

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_default_widget(&self) -> Option<Widget>;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_modal(&self) -> bool;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_pointing_to(&self) -> Option<gdk::Rectangle>;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_position(&self) -> PositionType;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_relative_to(&self) -> Option<Widget>;

    #[cfg_attr(feature = "v3_22", deprecated)]
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_transitions_enabled(&self) -> bool;

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn popdown(&self);

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn popup(&self);

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_constrain_to(&self, constraint: PopoverConstraint);

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_default_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_modal(&self, modal: bool);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_pointing_to(&self, rect: &gdk::Rectangle);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_position(&self, position: PositionType);

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_relative_to<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, relative_to: Q);

    #[cfg_attr(feature = "v3_22", deprecated)]
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_transitions_enabled(&self, transitions_enabled: bool);

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_constrain_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_relative_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg_attr(feature = "v3_22", deprecated)]
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_transitions_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Popover>> PopoverExt for O {
    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn bind_model<'a, 'b, P: IsA<gio::MenuModel> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b str>>>(&self, model: Q, action_namespace: R) {
        let model = model.into();
        let model = model.to_glib_none();
        let action_namespace = action_namespace.into();
        let action_namespace = action_namespace.to_glib_none();
        unsafe {
            ffi::gtk_popover_bind_model(self.to_glib_none().0, model.0, action_namespace.0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn get_constrain_to(&self) -> PopoverConstraint {
        unsafe {
            from_glib(ffi::gtk_popover_get_constrain_to(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn get_default_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_popover_get_default_widget(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_modal(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_modal(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_pointing_to(&self) -> Option<gdk::Rectangle> {
        unsafe {
            let mut rect = gdk::Rectangle::uninitialized();
            let ret = from_glib(ffi::gtk_popover_get_pointing_to(self.to_glib_none().0, rect.to_glib_none_mut().0));
            if ret { Some(rect) } else { None }
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_position(&self) -> PositionType {
        unsafe {
            from_glib(ffi::gtk_popover_get_position(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn get_relative_to(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_popover_get_relative_to(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn get_transitions_enabled(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_popover_get_transitions_enabled(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn popdown(&self) {
        unsafe {
            ffi::gtk_popover_popdown(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_22", feature = "dox"))]
    fn popup(&self) {
        unsafe {
            ffi::gtk_popover_popup(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn set_constrain_to(&self, constraint: PopoverConstraint) {
        unsafe {
            ffi::gtk_popover_set_constrain_to(self.to_glib_none().0, constraint.to_glib());
        }
    }

    #[cfg(any(feature = "v3_18", feature = "dox"))]
    fn set_default_widget<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, widget: Q) {
        let widget = widget.into();
        let widget = widget.to_glib_none();
        unsafe {
            ffi::gtk_popover_set_default_widget(self.to_glib_none().0, widget.0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_popover_set_modal(self.to_glib_none().0, modal.to_glib());
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_pointing_to(&self, rect: &gdk::Rectangle) {
        unsafe {
            ffi::gtk_popover_set_pointing_to(self.to_glib_none().0, rect.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_position(&self, position: PositionType) {
        unsafe {
            ffi::gtk_popover_set_position(self.to_glib_none().0, position.to_glib());
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn set_relative_to<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, relative_to: Q) {
        let relative_to = relative_to.into();
        let relative_to = relative_to.to_glib_none();
        unsafe {
            ffi::gtk_popover_set_relative_to(self.to_glib_none().0, relative_to.0);
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn set_transitions_enabled(&self, transitions_enabled: bool) {
        unsafe {
            ffi::gtk_popover_set_transitions_enabled(self.to_glib_none().0, transitions_enabled.to_glib());
        }
    }

    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"closed\0".as_ptr() as *const _,
                transmute(closed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    fn connect_property_constrain_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::constrain-to\0".as_ptr() as *const _,
                transmute(notify_constrain_to_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::modal\0".as_ptr() as *const _,
                transmute(notify_modal_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_pointing_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::pointing-to\0".as_ptr() as *const _,
                transmute(notify_pointing_to_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::position\0".as_ptr() as *const _,
                transmute(notify_position_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_12", feature = "dox"))]
    fn connect_property_relative_to_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::relative-to\0".as_ptr() as *const _,
                transmute(notify_relative_to_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_16", feature = "dox"))]
    fn connect_property_transitions_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::transitions-enabled\0".as_ptr() as *const _,
                transmute(notify_transitions_enabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn closed_trampoline<P>(this: *mut ffi::GtkPopover, f: glib_ffi::gpointer)
where P: IsA<Popover> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Popover::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_20", feature = "dox"))]
unsafe extern "C" fn notify_constrain_to_trampoline<P>(this: *mut ffi::GtkPopover, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Popover> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Popover::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
unsafe extern "C" fn notify_modal_trampoline<P>(this: *mut ffi::GtkPopover, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Popover> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Popover::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
unsafe extern "C" fn notify_pointing_to_trampoline<P>(this: *mut ffi::GtkPopover, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Popover> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Popover::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
unsafe extern "C" fn notify_position_trampoline<P>(this: *mut ffi::GtkPopover, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Popover> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Popover::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_12", feature = "dox"))]
unsafe extern "C" fn notify_relative_to_trampoline<P>(this: *mut ffi::GtkPopover, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Popover> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Popover::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_16", feature = "dox"))]
unsafe extern "C" fn notify_transitions_enabled_trampoline<P>(this: *mut ffi::GtkPopover, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Popover> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Popover::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for Popover {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Popover")
    }
}
