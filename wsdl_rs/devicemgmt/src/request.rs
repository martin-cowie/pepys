// Generated by ./build_scripts/gen_code.pl from ./schemata/devicemgmt.wsdl.xml ---------------------
use yaserde_derive::*;
use soapenv::Header;

#[derive(Debug, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
    namespace = "tds: http://www.onvif.org/ver10/device/wsdl"
)]
pub struct Envelope {
    #[yaserde(prefix = "s", rename = "Header")]
    pub header: Option<Header>,

    #[yaserde(prefix = "s", rename = "Body")]
    pub body: Body,
}

#[derive(Debug, YaDeserialize)]
#[yaserde(
    prefix = "s",
    namespace = "s: http://www.w3.org/2003/05/soap-envelope",
)]
pub enum Body {
    Unknown, // Requirement of `Default` impl, required by YaDeserialize

        #[yaserde(prefix = "tds")]
    AddIPAddressFilter(super::AddIPAddressFilter),

    #[yaserde(prefix = "tds")]
    AddScopes(super::AddScopes),

    #[yaserde(prefix = "tds")]
    CreateCertificate(super::CreateCertificate),

    #[yaserde(prefix = "tds")]
    CreateDot1XConfiguration(super::CreateDot1XConfiguration),

    #[yaserde(prefix = "tds")]
    CreateStorageConfiguration(super::CreateStorageConfiguration),

    #[yaserde(prefix = "tds")]
    CreateUsers(super::CreateUsers),

    #[yaserde(prefix = "tds")]
    DeleteCertificates(super::DeleteCertificates),

    #[yaserde(prefix = "tds")]
    DeleteDot1XConfiguration(super::DeleteDot1XConfiguration),

    #[yaserde(prefix = "tds")]
    DeleteGeoLocation(super::DeleteGeoLocation),

    #[yaserde(prefix = "tds")]
    DeleteStorageConfiguration(super::DeleteStorageConfiguration),

    #[yaserde(prefix = "tds")]
    DeleteUsers(super::DeleteUsers),

    #[yaserde(prefix = "tds")]
    GetAccessPolicy(super::GetAccessPolicy),

    #[yaserde(prefix = "tds")]
    GetAuthFailureWarningConfiguration(super::GetAuthFailureWarningConfiguration),

    #[yaserde(prefix = "tds")]
    GetAuthFailureWarningOptions(super::GetAuthFailureWarningOptions),

    #[yaserde(prefix = "tds")]
    GetCACertificates(super::GetCACertificates),

    #[yaserde(prefix = "tds")]
    GetCapabilities(super::GetCapabilities),

    #[yaserde(prefix = "tds")]
    GetCertificateInformation(super::GetCertificateInformation),

    #[yaserde(prefix = "tds")]
    GetCertificates(super::GetCertificates),

    #[yaserde(prefix = "tds")]
    GetCertificatesStatus(super::GetCertificatesStatus),

    #[yaserde(prefix = "tds")]
    GetClientCertificateMode(super::GetClientCertificateMode),

    #[yaserde(prefix = "tds")]
    GetDNS(super::GetDNS),

    #[yaserde(prefix = "tds")]
    GetDPAddresses(super::GetDPAddresses),

    #[yaserde(prefix = "tds")]
    GetDeviceInformation(super::GetDeviceInformation),

    #[yaserde(prefix = "tds")]
    GetDiscoveryMode(super::GetDiscoveryMode),

    #[yaserde(prefix = "tds")]
    GetDot11Capabilities(super::GetDot11Capabilities),

    #[yaserde(prefix = "tds")]
    GetDot11Status(super::GetDot11Status),

    #[yaserde(prefix = "tds")]
    GetDot1XConfiguration(super::GetDot1XConfiguration),

    #[yaserde(prefix = "tds")]
    GetDot1XConfigurations(super::GetDot1XConfigurations),

    #[yaserde(prefix = "tds")]
    GetDynamicDNS(super::GetDynamicDNS),

    #[yaserde(prefix = "tds")]
    GetEndpointReference(super::GetEndpointReference),

    #[yaserde(prefix = "tds")]
    GetGeoLocation(super::GetGeoLocation),

    #[yaserde(prefix = "tds")]
    GetHostname(super::GetHostname),

    #[yaserde(prefix = "tds")]
    GetIPAddressFilter(super::GetIPAddressFilter),

    #[yaserde(prefix = "tds")]
    GetNTP(super::GetNTP),

    #[yaserde(prefix = "tds")]
    GetNetworkDefaultGateway(super::GetNetworkDefaultGateway),

    #[yaserde(prefix = "tds")]
    GetNetworkInterfaces(super::GetNetworkInterfaces),

    #[yaserde(prefix = "tds")]
    GetNetworkProtocols(super::GetNetworkProtocols),

    #[yaserde(prefix = "tds")]
    GetPasswordComplexityConfiguration(super::GetPasswordComplexityConfiguration),

    #[yaserde(prefix = "tds")]
    GetPasswordComplexityOptions(super::GetPasswordComplexityOptions),

    #[yaserde(prefix = "tds")]
    GetPasswordHistoryConfiguration(super::GetPasswordHistoryConfiguration),

    #[yaserde(prefix = "tds")]
    GetPkcs10Request(super::GetPkcs10Request),

    #[yaserde(prefix = "tds")]
    GetRelayOutputs(super::GetRelayOutputs),

    #[yaserde(prefix = "tds")]
    GetRemoteDiscoveryMode(super::GetRemoteDiscoveryMode),

    #[yaserde(prefix = "tds")]
    GetRemoteUser(super::GetRemoteUser),

    #[yaserde(prefix = "tds")]
    GetScopes(super::GetScopes),

    #[yaserde(prefix = "tds")]
    GetServiceCapabilities(super::GetServiceCapabilities),

    #[yaserde(prefix = "tds")]
    GetServices(super::GetServices),

    #[yaserde(prefix = "tds")]
    GetStorageConfiguration(super::GetStorageConfiguration),

    #[yaserde(prefix = "tds")]
    GetStorageConfigurations(super::GetStorageConfigurations),

    #[yaserde(prefix = "tds")]
    GetSystemBackup(super::GetSystemBackup),

    #[yaserde(prefix = "tds")]
    GetSystemDateAndTime(super::GetSystemDateAndTime),

    #[yaserde(prefix = "tds")]
    GetSystemLog(super::GetSystemLog),

    #[yaserde(prefix = "tds")]
    GetSystemSupportInformation(super::GetSystemSupportInformation),

    #[yaserde(prefix = "tds")]
    GetSystemUris(super::GetSystemUris),

    #[yaserde(prefix = "tds")]
    GetUsers(super::GetUsers),

    #[yaserde(prefix = "tds")]
    GetWsdlUrl(super::GetWsdlUrl),

    #[yaserde(prefix = "tds")]
    GetZeroConfiguration(super::GetZeroConfiguration),

    #[yaserde(prefix = "tds")]
    LoadCACertificates(super::LoadCACertificates),

    #[yaserde(prefix = "tds")]
    LoadCertificateWithPrivateKey(super::LoadCertificateWithPrivateKey),

    #[yaserde(prefix = "tds")]
    LoadCertificates(super::LoadCertificates),

    #[yaserde(prefix = "tds")]
    RemoveIPAddressFilter(super::RemoveIPAddressFilter),

    #[yaserde(prefix = "tds")]
    RemoveScopes(super::RemoveScopes),

    #[yaserde(prefix = "tds")]
    RestoreSystem(super::RestoreSystem),

    #[yaserde(prefix = "tds")]
    ScanAvailableDot11Networks(super::ScanAvailableDot11Networks),

    #[yaserde(prefix = "tds")]
    SendAuxiliaryCommand(super::SendAuxiliaryCommand),

    #[yaserde(prefix = "tds")]
    SetAccessPolicy(super::SetAccessPolicy),

    #[yaserde(prefix = "tds")]
    SetAuthFailureWarningConfiguration(super::SetAuthFailureWarningConfiguration),

    #[yaserde(prefix = "tds")]
    SetCertificatesStatus(super::SetCertificatesStatus),

    #[yaserde(prefix = "tds")]
    SetClientCertificateMode(super::SetClientCertificateMode),

    #[yaserde(prefix = "tds")]
    SetDNS(super::SetDNS),

    #[yaserde(prefix = "tds")]
    SetDPAddresses(super::SetDPAddresses),

    #[yaserde(prefix = "tds")]
    SetDiscoveryMode(super::SetDiscoveryMode),

    #[yaserde(prefix = "tds")]
    SetDot1XConfiguration(super::SetDot1XConfiguration),

    #[yaserde(prefix = "tds")]
    SetDynamicDNS(super::SetDynamicDNS),

    #[yaserde(prefix = "tds")]
    SetGeoLocation(super::SetGeoLocation),

    #[yaserde(prefix = "tds")]
    SetHostname(super::SetHostname),

    #[yaserde(prefix = "tds")]
    SetHostnameFromDHCP(super::SetHostnameFromDHCP),

    #[yaserde(prefix = "tds")]
    SetIPAddressFilter(super::SetIPAddressFilter),

    #[yaserde(prefix = "tds")]
    SetNTP(super::SetNTP),

    #[yaserde(prefix = "tds")]
    SetNetworkDefaultGateway(super::SetNetworkDefaultGateway),

    #[yaserde(prefix = "tds")]
    SetNetworkInterfaces(super::SetNetworkInterfaces),

    #[yaserde(prefix = "tds")]
    SetNetworkProtocols(super::SetNetworkProtocols),

    #[yaserde(prefix = "tds")]
    SetPasswordComplexityConfiguration(super::SetPasswordComplexityConfiguration),

    #[yaserde(prefix = "tds")]
    SetPasswordHistoryConfiguration(super::SetPasswordHistoryConfiguration),

    #[yaserde(prefix = "tds")]
    SetRelayOutputSettings(super::SetRelayOutputSettings),

    #[yaserde(prefix = "tds")]
    SetRelayOutputState(super::SetRelayOutputState),

    #[yaserde(prefix = "tds")]
    SetRemoteDiscoveryMode(super::SetRemoteDiscoveryMode),

    #[yaserde(prefix = "tds")]
    SetRemoteUser(super::SetRemoteUser),

    #[yaserde(prefix = "tds")]
    SetScopes(super::SetScopes),

    #[yaserde(prefix = "tds")]
    SetStorageConfiguration(super::SetStorageConfiguration),

    #[yaserde(prefix = "tds")]
    SetSystemDateAndTime(super::SetSystemDateAndTime),

    #[yaserde(prefix = "tds")]
    SetSystemFactoryDefault(super::SetSystemFactoryDefault),

    #[yaserde(prefix = "tds")]
    SetUser(super::SetUser),

    #[yaserde(prefix = "tds")]
    SetZeroConfiguration(super::SetZeroConfiguration),

    #[yaserde(prefix = "tds")]
    StartFirmwareUpgrade(super::StartFirmwareUpgrade),

    #[yaserde(prefix = "tds")]
    StartSystemRestore(super::StartSystemRestore),

    #[yaserde(prefix = "tds")]
    SystemReboot(super::SystemReboot),

    #[yaserde(prefix = "tds")]
    UpgradeSystemFirmware(super::UpgradeSystemFirmware)
}

impl Default for Body {
    fn default() -> Self {
        Body::Unknown
    }
}
