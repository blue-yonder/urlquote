from setuptools import setup

def build_native(spec):
    # build rust library
    build = spec.add_external_build(
        cmd=['cargo', 'build', '--release'],
        path='.'
    )

    spec.add_cffi_module(
        module_path='urlquote._native',
        dylib=lambda: build.find_dylib('urlquote', in_path='target/release'),
        header_filename=lambda: build.find_header('native.h', in_path='./rust'),
        rtld_flags=['NOW', 'NODELETE']
    )

def readme():
    with open('README.md') as f:
        return f.read()

setup(
    name='urlquote',
    packages=['urlquote'],
    zip_safe=False,
    platforms='any',
    setup_requires=['milksnake', 'setuptools_scm'],
    install_requires=['milksnake'],
    use_scm_version=True,
    url='https://github.com/blue-yonder/urlquote',
    milksnake_tasks=[
        build_native
    ],
    author='Blue Yonder',
    author_email='oss@blue-yonder.com',
    license = 'MIT',
    description='Fast quoting and unquoting of urls.',
    long_description=readme(),
    long_description_content_type='text/markdown'
)