// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use Widget;
use ffi;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::object::ObjectExt;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib::signal::connect_raw;
use glib::translate::*;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use glib_ffi;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use gobject_ffi;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::boxed::Box as Box_;
use std::fmt;
#[cfg(any(feature = "v3_10", feature = "dox"))]
use std::mem::transmute;

glib_wrapper! {
    pub struct ListBoxRow(Object<ffi::GtkListBoxRow, ffi::GtkListBoxRowClass>): Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_list_box_row_get_type(),
    }
}

impl ListBoxRow {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn new() -> ListBoxRow {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_list_box_row_new()).downcast_unchecked()
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
impl Default for ListBoxRow {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ListBoxRowExt: 'static {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn changed(&self);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_activatable(&self) -> bool;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_header(&self) -> Option<Widget>;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_index(&self) -> i32;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_selectable(&self) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn is_selected(&self) -> bool;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_activatable(&self, activatable: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_header<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, header: Q);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_selectable(&self, selectable: bool);

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn emit_activate(&self);

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ListBoxRow>> ListBoxRowExt for O {
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn changed(&self) {
        unsafe {
            ffi::gtk_list_box_row_changed(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_get_activatable(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_header(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_row_get_header(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_list_box_row_get_index(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_get_selectable(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_is_selected(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_activatable(&self, activatable: bool) {
        unsafe {
            ffi::gtk_list_box_row_set_activatable(self.to_glib_none().0, activatable.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn set_header<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, header: Q) {
        let header = header.into();
        let header = header.to_glib_none();
        unsafe {
            ffi::gtk_list_box_row_set_header(self.to_glib_none().0, header.0);
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn set_selectable(&self, selectable: bool) {
        unsafe {
            ffi::gtk_list_box_row_set_selectable(self.to_glib_none().0, selectable.to_glib());
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"activate\0".as_ptr() as *const _,
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_10", feature = "dox"))]
    fn emit_activate(&self) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_ffi::GObject).emit("activate", &[]).unwrap() };
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_activatable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::activatable\0".as_ptr() as *const _,
                transmute(notify_activatable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_property_selectable_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.to_glib_none().0 as *mut _, b"notify::selectable\0".as_ptr() as *const _,
                transmute(notify_selectable_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_10", feature = "dox"))]
unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer)
where P: IsA<ListBoxRow> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ListBoxRow::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_activatable_trampoline<P>(this: *mut ffi::GtkListBoxRow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ListBoxRow> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ListBoxRow::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn notify_selectable_trampoline<P>(this: *mut ffi::GtkListBoxRow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ListBoxRow> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ListBoxRow::from_glib_borrow(this).downcast_unchecked())
}

impl fmt::Display for ListBoxRow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ListBoxRow")
    }
}
