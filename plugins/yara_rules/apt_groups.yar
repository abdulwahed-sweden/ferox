rule Ferox_APT_LateralMovement {
    meta:
        description = "APT style lateral movement tooling"
        severity = "medium"

    strings:
        $s1 = "Invoke-Mimikatz" nocase
        $s2 = "WmiPrvSE.exe" nocase
        $s3 = "psexec" nocase

    condition:
        2 of ($s*)
}
