use cpuidrs::*;
pub fn get_current_cpu_features() -> CpuInfo {
    cpuidrs::get_cpu_info()
}
