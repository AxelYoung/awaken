#[macro_export]
macro_rules! iterate_entities {
    // This matches any number of immutable
    ($world:expr,  
        [$($comps:ident),*],
        $logic:expr) => {{

        $(
            let comp_recur = $world.borrow_components::<$comps>().unwrap();
            let $comps = comp_recur.iter();
        )*

        let iter = multizip(($($comps),*));
        
        let iter = iter.filter_map(|($($comps),*)| Some((($($comps.as_ref()?),*))));

        for ($($comps),*) in iter {
            $logic($($comps),*);
        }
    }};

    // This matches any number of immutable and mutable components
    ($world:expr, 
        ($comp_mut:ident, $($comps_mut:ident),*), 
        $logic:expr) => {{

        let mut comp_mut = $world.borrow_components_mut::<$comp_mut>().unwrap();
        let comp_iter = comp_mut.iter_mut();

        $(
            let mut comp_mut_recur = $world.borrow_components_mut::<$comps_mut>().unwrap();
            let $comps_mut = comp_mut_recur.iter_mut();
        )*

        let iter = multizip((comp_iter, $($comps_mut),*));

        let iter = iter.filter_map(|(comp_iter, $($comps_mut),*)| Some(((comp_iter.as_mut()?, $($comps_mut.as_mut()?),*))));

        for (comp, $($comps_mut),*) in iter {
            $logic(comp, $($comps_mut),*);
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