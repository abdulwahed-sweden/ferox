rule Ferox_Ransomware_Generic {
    meta:
        description = "Detects ransomware note artifacts"
        severity = "high"

    strings:
        $a = "---BEGIN FEROCIOUS RANSOM NOTE---" nocase
        $b = "bitcoin" nocase
        $c = "decrypt" nocase

    condition:
        $a or (2 of ($b,$c))
}
