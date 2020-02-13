"use strict";

requirejs.config({
	baseUrl: "/static/js",
	paths: {
		dygraph: ["https://cdnjs.cloudflare.com/ajax/libs/dygraph/2.1.0/dygraph.min"],
		knockout: ["https://cdnjs.cloudflare.com/ajax/libs/knockout/3.4.2/knockout-min"],
		moment: ["https://cdnjs.cloudflare.com/ajax/libs/moment.js/2.19.1/moment.min"],
		reqwest: ["https://cdnjs.cloudflare.com/ajax/libs/reqwest/2.0.5/reqwest.min"],
		semantic: ["https://cdnjs.cloudflare.com/ajax/libs/semantic-ui/2.2.13/semantic.min"],
	},
	shim: {
		reqwest: {
			exports: "reqwest",
		},
	},
	waitSeconds: 15,
});

// Start the main application logic.
requirejs(
	["knockout", "Application"],
	function(ko, Application) {
		ko.applyBindings(new Application());
	},
	function(err) {
		console.log(err.requireType);

		if (err.requireType === "timeout") {
			console.log("modules: " + err.requireModules);
		}

		throw err;
	}
);
