use chrono::{DateTime, Local};
use prometheus_parse::Value;
use serde_derive::{Serialize,Deserialize};

#[derive(Debug)]
pub struct StoredNodeExporterValues {
    pub hostname_port: String,
    pub timestamp: DateTime<Local>,
    // sample.metric + labels
    pub metric_name: String,
    // sample.value type : types Counter and Gauge
    pub metric_type: String,
    // sample.value
    pub metric_value: f64,
}

fn main() {
    let body = reqwest::blocking::get("http://192.168.66.80:9300/metrics").unwrap()
        .text().unwrap();
    let lines: Vec<_> = body.lines().map(|s| Ok(s.to_owned())).collect();

    let metrics = prometheus_parse::Scrape::parse(lines.into_iter()).unwrap();
    //println!("{:#?}", metrics);
    let mut storednodeexportervalues = Vec::new();
    for sample in metrics.samples {
        //let mut label = sample.labels.values().cloned().collect::<Vec<String>>().join("_");
        let mut label_temp = sample.labels.values().cloned().collect::<Vec<String>>();
        label_temp.sort();
        let mut label = label_temp.join("_");
        label = if label.len() > 0 {
            format!("_{}", label)
        } else {
            label
        };

        match sample.value {
            Value::Counter(val) => {
                storednodeexportervalues.push(
                    StoredNodeExporterValues {
                        hostname_port: "blahblah".to_string(),
                        timestamp: Local::now(),
                        metric_name: format!("{}{}", sample.metric.to_string(), &label),
                        metric_type: "counter".to_string(),
                        metric_value: val,
                    }
                )
            },
            Value::Gauge(val) => {
                storednodeexportervalues.push(
                    StoredNodeExporterValues {
                        hostname_port: "blahblah".to_string(),
                        timestamp: Local::now(),
                        metric_name: format!("{}{}", sample.metric.to_string(), label),
                        metric_type: "gauge".to_string(),
                        metric_value: val,
                    }
                )
            },
            Value::Histogram(_val) => println!("histogram"),
            Value::Summary(_val) => println!("summary"),
            Value::Untyped(_val) => println!("untyped"),
        }
        //println!("{}", sample.metric);
    }
    println!("{:?}", storednodeexportervalues);


}
