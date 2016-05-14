// This file was generated by gir (4d68d19) from gir-files (11e0e6d)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct StyleProvider(Object<ffi::GtkStyleProvider>);

    match fn {
        get_type => || ffi::gtk_style_provider_get_type(),
    }
}

pub trait StyleProviderExt {
    //fn get_icon_factory(&self, path: /*Ignored*/&mut WidgetPath) -> /*Ignored*/Option<IconFactory>;

    //fn get_style(&self, path: /*Ignored*/&mut WidgetPath) -> /*Ignored*/Option<StyleProperties>;

    //fn get_style_property<T: IsA</*Ignored*/glib::ParamSpec>>(&self, path: /*Ignored*/&mut WidgetPath, state: StateFlags, pspec: &T) -> Option<glib::Value>;
}

impl<O: IsA<StyleProvider>> StyleProviderExt for O {
    //fn get_icon_factory(&self, path: /*Ignored*/&mut WidgetPath) -> /*Ignored*/Option<IconFactory> {
    //    unsafe { TODO: call ffi::gtk_style_provider_get_icon_factory() }
    //}

    //fn get_style(&self, path: /*Ignored*/&mut WidgetPath) -> /*Ignored*/Option<StyleProperties> {
    //    unsafe { TODO: call ffi::gtk_style_provider_get_style() }
    //}

    //fn get_style_property<T: IsA</*Ignored*/glib::ParamSpec>>(&self, path: /*Ignored*/&mut WidgetPath, state: StateFlags, pspec: &T) -> Option<glib::Value> {
    //    unsafe { TODO: call ffi::gtk_style_provider_get_style_property() }
    //}
}
