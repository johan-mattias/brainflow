use std::{thread, time::Duration};

use brainflow::{
    board_shim, brainflow_input_params::BrainFlowInputParamsBuilder, data_filter, BoardIds,
    DetrendOperations, WindowFunctions,
};

fn main() {
    brainflow::board_shim::enable_dev_board_logger().unwrap();
    let board_id = BoardIds::SyntheticBoard as i32;
    let eeg_channels = board_shim::get_eeg_channels(board_id).unwrap();
    let sampling_rate = board_shim::get_sampling_rate(board_id).unwrap();
    let nfft = data_filter::get_nearest_power_of_two(sampling_rate).unwrap();

    let params = BrainFlowInputParamsBuilder::default().build();
    let board_id = BoardIds::SyntheticBoard as i32;
    let board = board_shim::BoardShim::new(board_id, params).unwrap();

    board.prepare_session().unwrap();
    board.start_stream(45000, "").unwrap();
    thread::sleep(Duration::from_secs(5));
    board.stop_stream().unwrap();
    let mut data = board.get_board_data(None).unwrap();
    board.release_session().unwrap();

    data_filter::detrend(&mut data[eeg_channels[1]], DetrendOperations::Linear as i32).unwrap();
    let mut psd = data_filter::get_psd_welch(
        &mut data[eeg_channels[1]],
        nfft,
        nfft / 2,
        sampling_rate,
        WindowFunctions::BlackmanHarris as i32,
    )
    .unwrap();
    let band_power_alpha = data_filter::get_band_power(&mut psd, 7.0, 13.0).unwrap();
    let band_power_beta = data_filter::get_band_power(&mut psd, 14.0, 30.0).unwrap();
    println!(
        "band_power_alpha / band_power_beta = {}",
        band_power_alpha / band_power_beta
    );
}
