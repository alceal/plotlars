use plotlars_core::ir::data::ColumnData;
use plotlars_core::ir::trace::TraceIR;

pub(crate) fn extract_xy_pairs(x: &ColumnData, y: &ColumnData) -> Vec<(f64, f64)> {
    let xs = extract_f64_options(x);
    let ys = extract_f64_options(y);
    xs.into_iter()
        .zip(ys)
        .filter_map(|(x, y)| Some((x?, y?)))
        .collect()
}

/// For TimeSeriesPlot with string x-data, map dates to sequential indices.
/// Returns (indexed_points, x_labels). If x is numeric/datetime, falls back
/// to extract_xy_pairs and returns empty labels.
pub(crate) fn extract_timeseries_points(
    x: &ColumnData,
    y: &ColumnData,
) -> (Vec<(f64, f64)>, Vec<String>) {
    if matches!(x, ColumnData::String(_)) {
        let labels = extract_strings(x);
        let ys = extract_f64_options(y);
        let points: Vec<(f64, f64)> = labels
            .iter()
            .enumerate()
            .zip(ys)
            .filter_map(|((i, _), y_opt)| Some((i as f64, y_opt?)))
            .collect();
        (points, labels)
    } else {
        (extract_xy_pairs(x, y), vec![])
    }
}

pub(crate) fn extract_f64(data: &ColumnData) -> Vec<f64> {
    match data {
        ColumnData::Numeric(v) => v.iter().filter_map(|x| x.map(|v| v as f64)).collect(),
        ColumnData::DateTime(v) => v.iter().filter_map(|x| x.map(|v| v as f64)).collect(),
        ColumnData::String(_) => vec![],
    }
}

pub(crate) fn extract_strings(data: &ColumnData) -> Vec<String> {
    match data {
        ColumnData::String(v) => v.iter().map(|x| x.clone().unwrap_or_default()).collect(),
        ColumnData::Numeric(v) => v.iter().map(|x| format!("{}", x.unwrap_or(0.0))).collect(),
        ColumnData::DateTime(v) => v.iter().map(|x| format!("{}", x.unwrap_or(0))).collect(),
    }
}

fn extract_f64_options(data: &ColumnData) -> Vec<Option<f64>> {
    match data {
        ColumnData::Numeric(v) => v.iter().map(|x| x.map(|v| v as f64)).collect(),
        ColumnData::DateTime(v) => v.iter().map(|x| x.map(|v| v as f64)).collect(),
        ColumnData::String(_) => vec![],
    }
}

pub(crate) fn compute_numeric_ranges(traces: &[TraceIR]) -> (f64, f64, f64, f64) {
    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;

    for trace in traces {
        let pairs: Vec<(f64, f64)> = match trace {
            TraceIR::ScatterPlot(ir) => extract_xy_pairs(&ir.x, &ir.y),
            TraceIR::LinePlot(ir) => extract_xy_pairs(&ir.x, &ir.y),
            TraceIR::TimeSeriesPlot(ir) => {
                let (pts, _) = extract_timeseries_points(&ir.x, &ir.y);
                pts
            }
            TraceIR::Histogram(ir) => {
                let vals = extract_f64(&ir.x);
                if !vals.is_empty() {
                    let lo = vals.iter().cloned().fold(f64::INFINITY, f64::min);
                    let hi = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                    x_min = x_min.min(lo);
                    x_max = x_max.max(hi);
                    // y range will be set by bin counts later
                }
                continue;
            }
            TraceIR::CandlestickPlot(ir) => {
                let n = extract_strings(&ir.dates).len();
                if n > 0 {
                    x_min = x_min.min(-0.5);
                    x_max = x_max.max(n as f64 - 0.5);
                }
                let ohlc = extract_f64(&ir.open)
                    .into_iter()
                    .chain(extract_f64(&ir.high))
                    .chain(extract_f64(&ir.low))
                    .chain(extract_f64(&ir.close));
                for v in ohlc {
                    y_min = y_min.min(v);
                    y_max = y_max.max(v);
                }
                continue;
            }
            _ => continue,
        };
        for (x, y) in &pairs {
            x_min = x_min.min(*x);
            x_max = x_max.max(*x);
            y_min = y_min.min(*y);
            y_max = y_max.max(*y);
        }
    }

    // Add 5% margin
    let x_margin = (x_max - x_min).abs() * 0.05;
    let y_margin = (y_max - y_min).abs() * 0.05;
    if x_margin == 0.0 {
        (
            x_min - 1.0,
            x_max + 1.0,
            y_min - y_margin.max(1.0),
            y_max + y_margin.max(1.0),
        )
    } else {
        (
            x_min - x_margin,
            x_max + x_margin,
            y_min - y_margin.max(0.01),
            y_max + y_margin.max(0.01),
        )
    }
}

pub(crate) fn compute_bar_ranges(traces: &[TraceIR], stacked: bool) -> (usize, f64) {
    let categories = collect_bar_categories(traces);
    let n_categories = categories.len();

    if stacked {
        let mut pos_sums = vec![0.0f64; n_categories];
        let mut neg_sums = vec![0.0f64; n_categories];
        for trace in traces {
            if let TraceIR::BarPlot(ir) = trace {
                let labels = extract_strings(&ir.labels);
                let vals = extract_f64(&ir.values);
                let errs: Vec<f64> = ir.error.as_ref().map(extract_f64).unwrap_or_default();
                for (i, (label, &val)) in labels.iter().zip(vals.iter()).enumerate() {
                    if let Some(cat_idx) = categories.iter().position(|c| c == label) {
                        let e = errs.get(i).copied().unwrap_or(0.0);
                        if val >= 0.0 {
                            pos_sums[cat_idx] += val;
                            pos_sums[cat_idx] = pos_sums[cat_idx].max(pos_sums[cat_idx] + e);
                        } else {
                            neg_sums[cat_idx] += val;
                        }
                    }
                }
            }
        }
        let max_pos = pos_sums.iter().cloned().fold(0.0f64, f64::max);
        let min_neg = neg_sums.iter().cloned().fold(0.0f64, f64::min);
        (
            n_categories,
            if min_neg < 0.0 {
                max_pos.max(-min_neg)
            } else {
                max_pos
            },
        )
    } else {
        let mut max_val = 0.0f64;
        for trace in traces {
            if let TraceIR::BarPlot(ir) = trace {
                let vals = extract_f64(&ir.values);
                let errs: Vec<f64> = ir.error.as_ref().map(extract_f64).unwrap_or_default();
                for (i, v) in vals.iter().enumerate() {
                    let e = errs.get(i).copied().unwrap_or(0.0);
                    max_val = max_val.max(v + e);
                }
            }
        }
        (n_categories, max_val)
    }
}

pub(crate) fn collect_bar_categories(traces: &[TraceIR]) -> Vec<String> {
    for trace in traces {
        if let TraceIR::BarPlot(ir) = trace {
            return extract_strings(&ir.labels);
        }
    }
    vec![]
}

pub(crate) fn auto_compute_bins(values: &[f64]) -> (Vec<(f64, f64)>, Vec<usize>) {
    if values.is_empty() {
        return (vec![], vec![]);
    }
    let n_bins = ((values.len() as f64).sqrt().ceil() as usize).max(1);
    let min_val = values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max_val - min_val;
    let bin_size = if range == 0.0 {
        1.0
    } else {
        range / n_bins as f64
    };

    let mut bins = Vec::with_capacity(n_bins);
    let mut counts = vec![0usize; n_bins];

    for i in 0..n_bins {
        let start = min_val + i as f64 * bin_size;
        let end = start + bin_size;
        bins.push((start, end));
    }

    for &v in values {
        let idx = ((v - min_val) / bin_size).floor() as usize;
        let idx = idx.min(n_bins - 1);
        counts[idx] += 1;
    }

    (bins, counts)
}

pub(crate) fn compute_bins_from_ir(
    values: &[f64],
    bins_ir: &plotlars_core::ir::trace::BinsIR,
) -> (Vec<(f64, f64)>, Vec<usize>) {
    let n_bins = ((bins_ir.end - bins_ir.start) / bins_ir.size).ceil() as usize;
    let mut bins = Vec::with_capacity(n_bins);
    let mut counts = vec![0usize; n_bins];

    for i in 0..n_bins {
        let start = bins_ir.start + i as f64 * bins_ir.size;
        let end = start + bins_ir.size;
        bins.push((start, end));
    }

    for &v in values {
        if v < bins_ir.start || v > bins_ir.end {
            continue;
        }
        let idx = ((v - bins_ir.start) / bins_ir.size).floor() as usize;
        let idx = idx.min(n_bins - 1);
        counts[idx] += 1;
    }

    (bins, counts)
}

/// Collect date labels from the first TimeSeriesPlot trace with string x-data.
pub(crate) fn collect_timeseries_labels(traces: &[TraceIR]) -> Vec<String> {
    for trace in traces {
        if let TraceIR::TimeSeriesPlot(ir) = trace {
            if matches!(ir.x, ColumnData::String(_)) {
                return extract_strings(&ir.x);
            }
        }
    }
    vec![]
}

pub(crate) fn count_bar_groups(traces: &[TraceIR]) -> usize {
    traces
        .iter()
        .filter(|t| matches!(t, TraceIR::BarPlot(_)))
        .count()
}

pub(crate) fn is_horizontal_bar(traces: &[TraceIR]) -> bool {
    traces.iter().any(|t| {
        if let TraceIR::BarPlot(ir) = t {
            matches!(
                ir.orientation,
                Some(plotlars_core::components::Orientation::Horizontal)
            )
        } else {
            false
        }
    })
}

/// Collect date labels from the first CandlestickPlot trace with string date data.
pub(crate) fn collect_candlestick_labels(traces: &[TraceIR]) -> Vec<String> {
    for trace in traces {
        if let TraceIR::CandlestickPlot(ir) = trace {
            if matches!(ir.dates, ColumnData::String(_)) {
                return extract_strings(&ir.dates);
            }
        }
    }
    vec![]
}

pub(crate) fn histogram_max_count(traces: &[TraceIR]) -> f64 {
    let mut max_count = 0usize;
    for trace in traces {
        if let TraceIR::Histogram(ir) = trace {
            let values = extract_f64(&ir.x);
            let (_, counts) = if let Some(ref bins_ir) = ir.bins {
                compute_bins_from_ir(&values, bins_ir)
            } else {
                auto_compute_bins(&values)
            };
            for &c in &counts {
                max_count = max_count.max(c);
            }
        }
    }
    max_count as f64
}
