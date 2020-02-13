"use strict";

define(["knockout", "reqwest", "moment", "dygraph"], function(ko, reqwest, moment, Dygraph) {
	const Application = function() {
		this.availableMetrics = ko.observableArray([]);
		this.selectedMetric = ko.observable();
		this.loading = ko.observable(false);
		this.errorMessage = ko.observable("");

		this.isMetricInvalid = ko.pureComputed(() => this.selectedMetric() === undefined);
		this.isFormInvalid = ko.pureComputed(() => this.isMetricInvalid());
		this.isMetricPresent = ko.pureComputed(() => this.selectedMetric() !== undefined);
		this.isErrorMessagePresent = ko.pureComputed(() => this.errorMessage() !== "");

		const element = document.getElementById("chart");
		this.graph = new Dygraph(element, [[new Date(), 0]], {
			color: "#2185d0",
			fillGraph: true,
			labels: ["Time", "Speed"],
			ylabel: "Download speed (bytes/sec)",
			axes: {
				y: {
					axisLabelWidth: 90,
					axisLabelFormatter: function(d, gran) {
						const sizes = [1024, 1048576, 1073741824, 1099511627776];
						const names = [" b/s", " Kb/s", " Mb/s", " Gb/s"];

						for (const i in sizes) {
							if (d < sizes[i]) {
								return (d / sizes[i]).toFixed(1) + names[i];
							}
						}

						return (d / sizes[3]).toFixed(1) + names[3];
					},
				},
			},
			width: element.clientWidth,
			height: "480px",
		});

		this.loadMetrics();
	};

	Application.prototype.loadMetrics = function() {
		reqwest({
			url: "/api/v1/metrics",
			method: "POST",
			type: "json",
		})
			.then(response => {
				if (response.success) {
					this.availableMetrics(response.result);
				} else {
					this.errorMessage(response.message);
				}

				this.loading(false);
			})
			.fail((err, msg) => {
				this.loading(false);
				this.errorMessage(msg || err.responseText || "Server communication error");
			});

		this.loading(true);
	};

	Application.prototype.loadData = function() {
		reqwest({
			url: "/api/v1/data",
			method: "POST",
			contentType: "application/json",
			data: JSON.stringify({
				metric: this.selectedMetric(),
			}),
		})
			.then(response => {
				const data = response
					.split("\n")
					.filter(line => line !== "")
					.map(line => {
						const parts = line.split(";");
						const date = new Date(1000 * parseInt(parts[0]));
						const value = parseFloat(parts[1]);

						return [date, value];
					});

				this.graph.updateOptions({ file: data });
				this.loading(false);
			})
			.fail((err, msg) => {
				this.loading(false);
				this.errorMessage(msg || err.responseText || "Server communication error");
			});

		this.loading(true);
	};

	return Application;
});
