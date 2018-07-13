node("Debian_9_internet"){
    checkout scm
    try{
        step([$class: 'StashNotifier'])
        stage ("Setup environment"){
            sh '''#!/bin/bash -xe
                python3 -m venv /tmp/venv
                source /tmp/venv/bin/activate
            '''
        }
        stage("Build wheel"){
            sh '''#!/bin/bash -ex
                source /tmp/venv/bin/activate
                cd python
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