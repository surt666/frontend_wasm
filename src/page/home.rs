use crate::Urls;
use seed::{prelude::*, *};

pub fn view<Ms>(base_url: &Url) -> Node<Ms> {
    section![
        C!["hero", "is-medium", "ml-6"],
        div![
            C!["hero-body"],
            h1![C!["title", "is-size-1"], "NNEDL ADM",],
            a![
                attrs! {At::Href => "https://seed-rs.org/"},
                h2![C!["subtitle", "is-size-3"], "seed-rs.org"]
            ],
            a![
                C!["button", "is-primary", "mt-5", "is-size-5"],
                attrs! {At::Href => Urls::new(base_url).users()},
                strong!["Go to Users"],
            ],
        ],
	div![C!["fixed z-10 inset-0 overflow-y-auto"], attrs!{"hidden" => "true"},
	     div![C!["flex items-end justify-center min-h-screen pt-4 px-4 pb-20 text-center sm:block sm:p-0"],
		  div![C!["fixed inset-0 transition-opacity"], attrs!{"aria-hidden" => "true"},
		       div![C!["absolute inset-0 bg-gray-500 opacity-75"]],
		       span![C!["hidden sm:inline-block sm:align-middle sm:h-screen"], attrs!{"aria-hidden" => "true"}, "&#8203;"],
		       div![C!["inline-block align-bottom bg-white rounded-lg text-left overflow-hidden shadow-xl transform transition-all sm:my-8 sm:align-middle sm:max-w-lg sm:w-full"], attrs!{"role" => "dialog", "aria-modal" => "true", "aria-labelledby" => "modal-headline"},
			    div![C!["bg-white px-4 pt-5 pb-4 sm:p-6 sm:pb-4"],
				 div![C!["sm:flex sm:items-start"],
				      div![C!["mx-auto flex-shrink-0 flex items-center justify-center h-12 w-12 rounded-full bg-red-100 sm:mx-0 sm:h-10 sm:w-10"],
					   svg![C!["h-6 w-6 text-red-600"], attrs!{"xmlns" => "http://www.w3.org/2000/svg", "fill" => "none", "viewBox" => "0 0 24 24", "stroke" => "currentColor", "aria-hidden" => "true"},
						path![attrs!{"stroke-linecap" => "round", "stroke-linejoin" => "round", "stroke-width" => "2", "d" => "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"}]]],
           
				      div![C!["mt-3 text-center sm:mt-0 sm:ml-4 sm:text-left"],
					   h3![C!["text-lg leading-6 font-medium text-gray-900"], "Deactivate account"],          
					   div![C!["mt-2"],
						p![C!["text-sm text-gray-500"],
						   "Are you sure you want to deactivate your account? All of your data will be permanently removed. This action cannot be undone."]]]]],
			    div![C!["bg-gray-50 px-4 py-3 sm:px-6 sm:flex sm:flex-row-reverse"],
				 button![C!["w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-red-600 text-base font-medium text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500 sm:ml-3 sm:w-auto sm:text-sm"], "Deactivate"],
				 button![C!["mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm"], "Cancel"]]]]]]]
}
