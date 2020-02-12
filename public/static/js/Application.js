"use strict";

define(["knockout", "reqwest", "moment", "Chart"], function(ko, reqwest, moment, Chart) {
	const Application = function() {
		this.availableMetrics = ko.observableArray([]);
		this.selectedMetric = ko.observable();
		this.metricData = ko.observable("");
		this.loading = ko.observable(false);
		this.errorMessage = ko.observable("");

		this.isMetricInvalid = ko.pureComputed(() => this.selectedMetric() === undefined);
		this.isFormInvalid = ko.pureComputed(() => this.isMetricInvalid());
		this.isMetricPresent = ko.pureComputed(() => this.selectedMetric() !== undefined);
		this.isErrorMessagePresent = ko.pureComputed(() => this.errorMessage() !== "");

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
				this.metricData(response);
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
