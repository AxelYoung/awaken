#[macro_export]
macro_rules! iterate_entities {
    // This matches any number of immutable
    ($world:expr,  
        [$($components:ident),*],
        $function:expr) => {{

        $(
            let component_vec = $world.borrow_components::<$components>().unwrap();
            let $components = component_vec.iter();
        )*

        let iter = multizip(($($components),*));
        
        let filtered_iter = iter.filter_map(|($($components),*)| Some((($($components.as_ref()?),*))));

        for ($($components),*) in filtered_iter {
            $function($($components),*);
        }
    }};

    // This matches a single mutable component (multizip doesn't like when a single mutable iterator is passed into it)
    ($world:expr, 
        ($mut_components:ident), 
        $function:expr) => {{

        let mut mut_components_vec = $world.borrow_components_mut::<$mut_components>().unwrap();
        let $mut_components = mut_components_vec.iter_mut();

        let iter = $mut_components.filter_map(|($mut_components)| Some((($mut_components.as_mut()?))));

        for ($mut_components) in iter {
            $function($mut_components);
        }
    }};

    // This matches any number of mutable components greater than one
    ($world:expr, 
        ($($mut_components:ident),*), 
        $function:expr) => {{

        $(
            let mut mut_components_vec = $world.borrow_components_mut::<$mut_components>().unwrap();
            let $mut_components = mut_components_vec.iter_mut();
        )*

        let iter = multizip(($($mut_components),*));

        let iter = iter.filter_map(|($($mut_components),*)| Some((($($mut_components.as_mut()?),*))));

        for ($($mut_components),*) in iter {
            $function($($mut_components),*);
        }
    }};
    
    // This matches any number of immutable and mutable components
    ($world:expr, 
        [$($comps:ident),*],
        ($($comps_mut:ident),*), 
        $logic:expr) => {{

        $(
            let comp_recur = $world.borrow_components::<$comps>().unwrap();
            let $comps = comp_recur.iter();
        )*

        $(
            let mut comp_mut_recur = $world.borrow_components_mut::<$comps_mut>().unwrap();
            let $comps_mut = comp_mut_recur.iter_mut();
        )*

        let iter = multizip(($($comps),*, $($comps_mut),*));

        let iter = iter.filter_map(|($($comps),*, $($comps_mut),*)| Some((($($comps.as_ref()?),*, $($comps_mut.as_mut()?),*))));

        for ($($comps),*, $($comps_mut),*) in iter {
            $logic($($comps),*, $($comps_mut),*);
        }
    }};
}

#[macro_export]
macro_rules! iterate_entities_with_id {
    // This matches any number of immutable
    ($world:expr,  
        [$($components:ident),*],
        $function:expr) => {{

        $(
            let component_vec = $world.borrow_components::<$components>().unwrap();
            let $components = component_vec.iter();
        )*

        let iter = multizip(($($components),*));
        
        let filtered_iter = iter.enumerate().filter_map(|(id, ($($components),*))| Some(((id, $($components.as_ref()?),*))));

        for (id, $($components),*) in filtered_iter {
            $function(id, $($components),*);
        }
    }};

    // This matches a single mutable component (multizip doesn't like when a single mutable iterator is passed into it)
    ($world:expr, 
        ($mut_components:ident), 
        $function:expr) => {{

        let mut mut_components_vec = $world.borrow_components_mut::<$mut_components>().unwrap();
        let $mut_components = mut_components_vec.iter_mut();

        let iter = $mut_components.enumerate().filter_map(|(id, $mut_components)| Some(((id, $mut_components.as_mut()?))));

        for (id, $mut_components) in iter {
            $function($mut_components);
        }
    }};

    // This matches any number of mutable components greater than one
    ($world:expr, 
        ($($mut_components:ident),*), 
        $function:expr) => {{

        $(
            let mut mut_components_vec = $world.borrow_components_mut::<$mut_components>().unwrap();
            let $mut_components = mut_components_vec.iter_mut();
        )*

        let iter = multizip(($($mut_components),*));

        let iter = iter.enumerate().filter_map(|(id, ($($mut_components),*))| Some(((id, $($mut_components.as_mut()?),*))));

        for (id, $($mut_components),*) in iter {
            $function(id, $($mut_components),*);
        }
    }};
    
    // This matches any number of immutable and mutable components
    ($world:expr, 
        [$($comps:ident),*],
        ($($comps_mut:ident),*), 
        $logic:expr) => {{

        $(
            let comp_recur = $world.borrow_components::<$comps>().unwrap();
            let $comps = comp_recur.iter();
        )*

        $(
            let mut comp_mut_recur = $world.borrow_components_mut::<$comps_mut>().unwrap();
            let $comps_mut = comp_mut_recur.iter_mut();
        )*

        let iter = multizip(($($comps),*, $($comps_mut),*));

        let iter = iter.enumerate().filter_map(|(id, ($($comps),*, $($comps_mut),*))| Some(((id, $($comps.as_ref()?),*, $($comps_mut.as_mut()?),*))));

        for (id, $($comps),*, $($comps_mut),*) in iter {
            $logic(id, $($comps),*, $($comps_mut),*);
        }
    }};
}