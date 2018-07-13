from setuptools import setup

def build_native(spec):
    # build rust library
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='./rust'
    )

    spec.add_cffi_module(
        module_path='urlquote._native',
        dylib=lambda: build.find_dylib('urlquote', in_path='target/release'),
        header_filename=lambda: build.find_header('native.h', in_path='.'),
        rtld_flags=['NOW', 'NODELETE']
    )

setup(
    name='urlquote',
    version='0.1.0',
    packages=['urlquote'],
    zip_safe=False,
    platforms='any',
    setup_requires=['milksnake'],
    install_requires=['milksnake'],
    # url='https://github.com/mypackage.git',
    milksnake_tasks=[
        build_native
    ],
    author='Markus Klein',
    author_email='markus.klein@blue-yonder.com',
    description='fast masking and unmasking of urls',
)