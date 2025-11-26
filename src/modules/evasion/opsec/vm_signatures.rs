//! Virtual Machine Detection Signatures
//!
//! Contains signatures and detection methods for various VM platforms.
//!
//! MITRE ATT&CK: T1497 (Virtualization/Sandbox Evasion)
//!
//! **SECURITY NOTICE**: AUTHORIZED USE ONLY

use serde::{Deserialize, Serialize};

/// Known VM types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum VmType {
    /// VMware Workstation/Fusion/ESXi
    VMware,
    /// Oracle VirtualBox
    VirtualBox,
    /// Microsoft Hyper-V
    HyperV,
    /// QEMU emulator
    QEMU,
    /// Linux KVM
    KVM,
    /// Xen hypervisor
    Xen,
    /// Parallels Desktop
    Parallels,
    /// Amazon EC2
    AmazonEC2,
    /// Google Cloud Platform
    GoogleCloud,
    /// Microsoft Azure
    Azure,
    /// DigitalOcean
    DigitalOcean,
    /// Docker container
    Docker,
    /// Windows Subsystem for Linux
    WSL,
    /// Unknown/unidentified VM
    Unknown,
}

impl VmType {
    /// Get VM vendor name
    pub fn vendor(&self) -> &'static str {
        match self {
            Self::VMware => "VMware",
            Self::VirtualBox => "Oracle",
            Self::HyperV => "Microsoft",
            Self::QEMU => "QEMU",
            Self::KVM => "Linux KVM",
            Self::Xen => "Xen Project",
            Self::Parallels => "Parallels",
            Self::AmazonEC2 => "Amazon AWS",
            Self::GoogleCloud => "Google Cloud",
            Self::Azure => "Microsoft Azure",
            Self::DigitalOcean => "DigitalOcean",
            Self::Docker => "Docker",
            Self::WSL => "Microsoft WSL",
            Self::Unknown => "Unknown",
        }
    }

    /// Get VM product name
    pub fn product_name(&self) -> &'static str {
        match self {
            Self::VMware => "VMware Workstation/Fusion/ESXi",
            Self::VirtualBox => "Oracle VirtualBox",
            Self::HyperV => "Microsoft Hyper-V",
            Self::QEMU => "QEMU",
            Self::KVM => "Kernel-based Virtual Machine",
            Self::Xen => "Xen Hypervisor",
            Self::Parallels => "Parallels Desktop",
            Self::AmazonEC2 => "Amazon EC2",
            Self::GoogleCloud => "Google Compute Engine",
            Self::Azure => "Azure Virtual Machine",
            Self::DigitalOcean => "DigitalOcean Droplet",
            Self::Docker => "Docker Container",
            Self::WSL => "Windows Subsystem for Linux",
            Self::Unknown => "Unknown VM",
        }
    }

    /// Is this a cloud environment?
    pub fn is_cloud(&self) -> bool {
        matches!(
            self,
            Self::AmazonEC2 | Self::GoogleCloud | Self::Azure | Self::DigitalOcean
        )
    }

    /// Is this a container environment?
    pub fn is_container(&self) -> bool {
        matches!(self, Self::Docker | Self::WSL)
    }

    /// Likelihood of being an analysis environment (1-10)
    pub fn analysis_risk(&self) -> u8 {
        match self {
            Self::VirtualBox => 8, // Very common for malware analysis
            Self::VMware => 6,    // Common but also enterprise
            Self::QEMU => 9,      // Very common for analysis
            Self::KVM => 7,
            Self::HyperV => 4,    // Often legitimate
            Self::AmazonEC2 | Self::GoogleCloud | Self::Azure => 3,
            Self::Docker => 5,
            Self::WSL => 3,
            Self::Xen => 5,
            Self::Parallels => 4,
            Self::DigitalOcean => 4,
            Self::Unknown => 5,
        }
    }
}

/// VM detection signature
#[derive(Debug, Clone)]
pub struct VmSignature {
    /// Type of VM
    pub vm_type: VmType,
    /// Process names to look for
    pub processes: &'static [&'static str],
    /// Service names to look for
    pub services: &'static [&'static str],
    /// File paths to check
    pub files: &'static [&'static str],
    /// Registry keys to check (Windows)
    pub registry_keys: &'static [&'static str],
    /// MAC address prefixes (OUI)
    pub mac_prefixes: &'static [&'static str],
    /// CPUID vendor string
    pub cpuid_vendor: Option<&'static str>,
    /// BIOS/SMBIOS strings
    pub bios_strings: &'static [&'static str],
    /// Driver files to check
    pub drivers: &'static [&'static str],
}

/// Get all VM signatures
pub fn get_vm_signatures() -> Vec<VmSignature> {
    vec![
        // VMware
        VmSignature {
            vm_type: VmType::VMware,
            processes: &[
                "vmtoolsd.exe",
                "vmwaretray.exe",
                "vmwareuser.exe",
                "vmacthlp.exe",
            ],
            services: &["VMTools", "vmvss", "vmhgfs"],
            files: &[
                r"C:\Windows\System32\drivers\vmhgfs.sys",
                r"C:\Windows\System32\drivers\vmmouse.sys",
                r"C:\Windows\System32\drivers\vmci.sys",
            ],
            registry_keys: &[
                r"HKLM\SOFTWARE\VMware, Inc.\VMware Tools",
                r"HKLM\HARDWARE\DEVICEMAP\Scsi\Scsi Port 0\Scsi Bus 0\Target Id 0\Logical Unit Id 0",
            ],
            mac_prefixes: &["00:0C:29", "00:50:56", "00:05:69"],
            cpuid_vendor: Some("VMwareVMware"),
            bios_strings: &["VMware", "Virtual Platform"],
            drivers: &["vmhgfs.sys", "vmmouse.sys", "vmci.sys", "vmusbmouse.sys"],
        },
        // VirtualBox
        VmSignature {
            vm_type: VmType::VirtualBox,
            processes: &["VBoxService.exe", "VBoxTray.exe"],
            services: &["VBoxService"],
            files: &[
                r"C:\Windows\System32\drivers\VBoxMouse.sys",
                r"C:\Windows\System32\drivers\VBoxGuest.sys",
                r"C:\Windows\System32\drivers\VBoxSF.sys",
                r"C:\Windows\System32\VBoxControl.exe",
            ],
            registry_keys: &[
                r"HKLM\SOFTWARE\Oracle\VirtualBox Guest Additions",
                r"HKLM\HARDWARE\ACPI\DSDT\VBOX__",
            ],
            mac_prefixes: &["08:00:27"],
            cpuid_vendor: Some("VBoxVBoxVBox"),
            bios_strings: &["VirtualBox", "VBOX", "Oracle VM"],
            drivers: &[
                "VBoxMouse.sys",
                "VBoxGuest.sys",
                "VBoxSF.sys",
                "VBoxVideo.sys",
            ],
        },
        // Hyper-V
        VmSignature {
            vm_type: VmType::HyperV,
            processes: &["vmms.exe", "vmwp.exe"],
            services: &["vmms", "vmicvss", "vmicshutdown"],
            files: &[
                r"C:\Windows\System32\drivers\vmbus.sys",
                r"C:\Windows\System32\drivers\VMBusHID.sys",
            ],
            registry_keys: &[r"HKLM\SOFTWARE\Microsoft\Virtual Machine\Guest\Parameters"],
            mac_prefixes: &["00:15:5D"],
            cpuid_vendor: Some("Microsoft Hv"),
            bios_strings: &["Hyper-V", "Microsoft Corporation Virtual Machine"],
            drivers: &["vmbus.sys", "VMBusHID.sys", "storvsc.sys"],
        },
        // QEMU/KVM
        VmSignature {
            vm_type: VmType::QEMU,
            processes: &["qemu-ga.exe", "qemu-system"],
            services: &["QEMU Guest Agent"],
            files: &[r"C:\Program Files\QEMU\qemu-ga.exe"],
            registry_keys: &[r"HKLM\HARDWARE\DEVICEMAP\Scsi\Scsi Port 0"],
            mac_prefixes: &["52:54:00", "00:16:3E"],
            cpuid_vendor: Some("KVMKVMKVM"),
            bios_strings: &["QEMU", "Bochs", "SeaBIOS"],
            drivers: &["vioscsi.sys", "viostor.sys", "netkvm.sys"],
        },
        // KVM
        VmSignature {
            vm_type: VmType::KVM,
            processes: &[],
            services: &[],
            files: &["/dev/kvm", "/sys/hypervisor/type"],
            registry_keys: &[],
            mac_prefixes: &["52:54:00"],
            cpuid_vendor: Some("KVMKVMKVM"),
            bios_strings: &["KVM"],
            drivers: &["virtio"],
        },
        // Xen
        VmSignature {
            vm_type: VmType::Xen,
            processes: &["xenservice.exe"],
            services: &["xenevtchn", "xenvbd", "xennet"],
            files: &[r"C:\Windows\System32\drivers\xen*.sys"],
            registry_keys: &[r"HKLM\SOFTWARE\Xen"],
            mac_prefixes: &["00:16:3E"],
            cpuid_vendor: Some("XenVMMXenVMM"),
            bios_strings: &["Xen", "HVM domU"],
            drivers: &["xen.sys", "xennet.sys", "xenvbd.sys"],
        },
        // Parallels
        VmSignature {
            vm_type: VmType::Parallels,
            processes: &["prl_tools.exe", "prl_cc.exe"],
            services: &["prl_tools"],
            files: &[r"C:\Program Files\Parallels\Parallels Tools"],
            registry_keys: &[r"HKLM\SOFTWARE\Parallels\Tools"],
            mac_prefixes: &["00:1C:42"],
            cpuid_vendor: Some("prl hyperv  "),
            bios_strings: &["Parallels", "Virtual Platform"],
            drivers: &["prl_fs.sys", "prl_memdev.sys"],
        },
        // AWS EC2
        VmSignature {
            vm_type: VmType::AmazonEC2,
            processes: &[],
            services: &["AmazonSSMAgent", "AmazonCloudWatchAgent"],
            files: &[
                r"C:\Program Files\Amazon\SSM",
                r"C:\Program Files\Amazon\EC2ConfigService",
                "/sys/hypervisor/uuid",
            ],
            registry_keys: &[r"HKLM\SOFTWARE\Amazon\EC2Config"],
            mac_prefixes: &[],
            cpuid_vendor: None,
            bios_strings: &["Amazon EC2", "Xen"],
            drivers: &["xenvbd.sys", "xennet.sys"],
        },
        // Azure
        VmSignature {
            vm_type: VmType::Azure,
            processes: &["WindowsAzureGuestAgent.exe", "WaAppAgent.exe"],
            services: &["WindowsAzureGuestAgent", "RdAgent"],
            files: &[r"C:\WindowsAzure", r"C:\Packages\Plugins"],
            registry_keys: &[r"HKLM\SOFTWARE\Microsoft\Windows Azure"],
            mac_prefixes: &["00:0D:3A", "00:17:FA"],
            cpuid_vendor: Some("Microsoft Hv"),
            bios_strings: &["Microsoft Corporation Virtual Machine"],
            drivers: &["vmbus.sys"],
        },
        // Google Cloud
        VmSignature {
            vm_type: VmType::GoogleCloud,
            processes: &["google_guest_agent.exe", "google_osconfig_agent.exe"],
            services: &["GCEAgent"],
            files: &[
                r"C:\Program Files\Google\Compute Engine",
                "/usr/share/google",
            ],
            registry_keys: &[r"HKLM\SOFTWARE\Google\ComputeEngine"],
            mac_prefixes: &["42:01:0A"],
            cpuid_vendor: None,
            bios_strings: &["Google", "Google Compute Engine"],
            drivers: &[],
        },
        // DigitalOcean
        VmSignature {
            vm_type: VmType::DigitalOcean,
            processes: &["do-agent"],
            services: &["do-agent"],
            files: &["/var/lib/cloud/instance", "/etc/cloud/cloud.cfg.d"],
            registry_keys: &[],
            mac_prefixes: &[],
            cpuid_vendor: None,
            bios_strings: &["DigitalOcean"],
            drivers: &[],
        },
        // Docker
        VmSignature {
            vm_type: VmType::Docker,
            processes: &[],
            services: &[],
            files: &["/.dockerenv", "/proc/1/cgroup"],
            registry_keys: &[],
            mac_prefixes: &["02:42"],
            cpuid_vendor: None,
            bios_strings: &[],
            drivers: &[],
        },
        // WSL
        VmSignature {
            vm_type: VmType::WSL,
            processes: &[],
            services: &[],
            files: &["/proc/sys/fs/binfmt_misc/WSLInterop"],
            registry_keys: &[],
            mac_prefixes: &[],
            cpuid_vendor: None,
            bios_strings: &["Microsoft"],
            drivers: &[],
        },
    ]
}

/// Get VM signature by type
pub fn get_signature_for_type(vm_type: VmType) -> Option<VmSignature> {
    get_vm_signatures()
        .into_iter()
        .find(|s| s.vm_type == vm_type)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vm_signatures_not_empty() {
        let sigs = get_vm_signatures();
        assert!(!sigs.is_empty());
        assert!(sigs.len() >= 10);
    }

    #[test]
    fn test_vm_type_vendor() {
        assert_eq!(VmType::VMware.vendor(), "VMware");
        assert_eq!(VmType::VirtualBox.vendor(), "Oracle");
        assert_eq!(VmType::HyperV.vendor(), "Microsoft");
    }

    #[test]
    fn test_vm_type_product_name() {
        let name = VmType::AmazonEC2.product_name();
        assert!(name.contains("EC2"));
    }

    #[test]
    fn test_vm_type_is_cloud() {
        assert!(VmType::AmazonEC2.is_cloud());
        assert!(VmType::Azure.is_cloud());
        assert!(VmType::GoogleCloud.is_cloud());
        assert!(!VmType::VMware.is_cloud());
        assert!(!VmType::VirtualBox.is_cloud());
    }

    #[test]
    fn test_vm_type_is_container() {
        assert!(VmType::Docker.is_container());
        assert!(VmType::WSL.is_container());
        assert!(!VmType::VMware.is_container());
    }

    #[test]
    fn test_vm_type_analysis_risk() {
        // VirtualBox should have higher risk than cloud
        assert!(VmType::VirtualBox.analysis_risk() > VmType::AmazonEC2.analysis_risk());
        // QEMU should have high risk
        assert!(VmType::QEMU.analysis_risk() >= 8);
    }

    #[test]
    fn test_get_signature_for_type() {
        let sig = get_signature_for_type(VmType::VMware);
        assert!(sig.is_some());
        assert_eq!(sig.unwrap().vm_type, VmType::VMware);
    }

    #[test]
    fn test_vmware_signature_content() {
        let sig = get_signature_for_type(VmType::VMware).unwrap();
        assert!(!sig.processes.is_empty());
        assert!(!sig.mac_prefixes.is_empty());
        assert!(sig.cpuid_vendor.is_some());
    }
}
