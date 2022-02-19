sources = [
    'main.rs'
]

cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml', '--target-dir', meson.project_build_root() / 'src', '--release' ]
# cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]

# cargo_options += [ '--release' ]
rust_target = 'release' 

cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]
cargo = find_program('cargo')

cargo_release = custom_target(
    'cargo-build',
    build_by_default: true,
    build_always_stale: true,
    output: meson.project_name(),
    console: true,
    install: true,
    install_dir: get_option('bindir'),
    command: [
        'env',
        cargo_env,
        cargo, 'build',
        cargo_options,
        '&&',
        'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
    ]
)
