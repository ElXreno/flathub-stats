%global commit 4cbd78e26a0418f9d9664ce2a39c1170e0640169
%global shortcommit %(c=%{commit}; echo ${c:0:7})

Name:           flathub-stats
Version:        0~7.git%{shortcommit}
Release:        1%{?dist}
Summary:        Utility for fast grepping stats from Flathub

License:        MPLv2.0
URL:            https://github.com/ElXreno/flathub-stats
Source:         %{url}/archive/%{commit}/%{name}-%{shortcommit}.tar.gz

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging

%description
%{summary}.

%prep
%autosetup -n %{name}-%{commit}
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build

%install
%cargo_install

%check
%cargo_test

%files
%license LICENSE
%doc README.md
%{_bindir}/flathub-stats

%changelog
* Sun May 24 2020 ElXreno <elxreno@gmail.com> - 0~7.git4cbd78e-1
- Update to the latest snapshot

* Sun May 24 2020 Igor Raits <ignatenkobrain@fedoraproject.org> - 0~6.git3933448-1
- Update to the latest snapshot

* Sat May 23 2020 Igor Raits <ignatenkobrain@fedoraproject.org> - 0~1.git8710138-1
- Initial package

