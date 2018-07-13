node("Debian_9_internet"){
    checkout scm
    try{
        step([$class: 'StashNotifier'])
        stage ("Setup environment"){
            sh '''#!/bin/bash -xe
                virtualenv --python=python3.6 /tmp/venv
                source /tmp/venv/bin/activate
                pip install pytest
            '''
        }
        stage("Build wheel"){
            sh '''#!/bin/bash -ex
                source /tmp/venv/bin/activate
                python setup.py sdist bdist_wheel
            '''
        }
        stage("Run tests"){
            sh '''#!/bin/bash -ex
                pytest test.py
            '''
        }
        currentBuild.result = 'SUCCESS'
    } catch(e) {
        currentBuild.result = 'FAILED'
    } finally {
        step([$class: 'StashNotifier'])
    }
}