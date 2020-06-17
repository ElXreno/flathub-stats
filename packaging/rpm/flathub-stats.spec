%global debug_package %{nil}

Name:           flathub-stats
Version:        0.1.0
Release:        1%{?dist}
Summary:        Utility for fast grepping stats from Flathub

License:        MPLv2.0
URL:            https://github.com/ElXreno/flathub-stats
Source:         %{url}/releases/download/v%{version}/%{name}-sources.tar.gz

ExclusiveArch:  %{rust_arches}

BuildRequires:  rust-packaging

BuildRequires:  pkgconfig(openssl)
BuildRequires:  pkgconfig(sqlite3)

%description
%{summary}.

%prep
%autosetup -c

# Let's say cargo use vendored sources
mkdir ~/.cargo
cat > ~/.cargo/config <<EOF
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "$(pwd)/vendor"
EOF


%build
cargo build --release --locked
strip target/release/%{name}


%install
install -m 0755 -Dp target/release/%{name} %{buildroot}%{_bindir}/%{name}


%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}

%changelog
* Wed Jun 17 2020 ElXreno <elxreno@gmail.com> - 0.1.0-1
- Updated to version 0.1.0

* Sun May 24 2020 ElXreno <elxreno@gmail.com> - 0~7.git4cbd78e-1
- Update to the latest snapshot

* Sun May 24 2020 Igor Raits <ignatenkobrain@fedoraproject.org> - 0~6.git3933448-1
- Update to the latest snapshot

* Sat May 23 2020 Igor Raits <ignatenkobrain@fedoraproject.org> - 0~1.git8710138-1
- Initial package

