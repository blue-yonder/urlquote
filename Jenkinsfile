node("Debian_9_internet"){
    checkout scm
    try{
        step([$class: 'StashNotifier'])
        stage ("Setup environment"){
            sh '''#!/bin/bash -xe
                virtualenv --python=python3.6 /tmp/venv
                source /tmp/venv/bin/activate

                pip install cffi
            '''
        }
        stage("Install Package"){
            sh '''#!/bin/bash -ex
                pip install -e .
            '''
        }
        stage("Run tests"){
            sh '''#!/bin/bash -ex
                source /tmp/venv/bin/activate
                pip install pytest
                pytest test.py
            '''
        }
        stage("Build wheel"){
            sh '''#!/bin/bash -ex
                source /tmp/venv/bin/activate
                python setup.py sdist bdist_wheel
            '''
        }
        currentBuild.result = 'SUCCESS'
    } catch(e) {
        currentBuild.result = 'FAILED'
    } finally {
        step([$class: 'StashNotifier'])
    }
}