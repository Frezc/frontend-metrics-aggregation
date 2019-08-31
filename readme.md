**This is only a workable example**

## Entries

`[POST] /resources`: Receive [performance entries](https://developer.mozilla.org/en-US/docs/Web/API/PerformanceEntry) from [frontend metric collector](https://github.com/Frezc/metrics-collector) and aggregate duration metric.
`[POST] /custom_metrics`: Receive custom metrics from [promjs](https://github.com/weaveworks/promjs) and [promjs-export](https://github.com/Frezc/promjs-export). Only aggregate counter now.
`[GET] /metrics`: Export all metrics for prometheus. 
