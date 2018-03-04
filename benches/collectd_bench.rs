extern crate collectd_plugin;
#[macro_use]
extern crate criterion;

use collectd_plugin::bindings::{data_set_t, data_source_t, value_list_t, value_t, ARR_LENGTH};
use collectd_plugin::{nanos_to_collectd, ValueList};
use std::os::raw::c_char;
use criterion::Criterion;
use std::ptr;

fn convert_to_value_list(c: &mut Criterion) {
    c.bench_function("convert_to_value_list", |b| {
        let empty: [c_char; ARR_LENGTH] = [0; ARR_LENGTH];
        let mut metric: [c_char; ARR_LENGTH] = [0; ARR_LENGTH];
        metric[0] = b'h' as c_char;
        metric[1] = b'o' as c_char;

        let mut name: [c_char; ARR_LENGTH] = [0; ARR_LENGTH];
        name[0] = b'h' as c_char;
        name[1] = b'i' as c_char;

        let val = data_source_t {
            name: name,
            type_: DS_TYPE_GAUGE as i32,
            min: 10.0,
            max: 11.0,
        };

        let mut v = vec![val];

        let conv = data_set_t {
            type_: metric,
            ds_num: 1,
            ds: v.as_mut_ptr(),
        };

        let mut vs = vec![value_t { gauge: 3.0 }];

        let list_t = value_list_t {
            values: vs.as_mut_ptr(),
            values_len: 1,
            time: nanos_to_collectd(1_000_000_000),
            interval: nanos_to_collectd(1_000_000_000),
            host: metric,
            plugin: name,
            plugin_instance: metric,
            type_: metric,
            type_instance: empty,
            meta: ptr::null_mut(),
        };
        b.iter(|| ValueList::from(&conv, &list_t))
    });
}

criterion_group!(benches, convert_to_value_list);
criterion_main!(benches);
