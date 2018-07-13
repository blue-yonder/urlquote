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
        if (env.BRANCH_NAME == "master") { 
            stage ('upload') {
                withCredentials([usernamePassword(
                    credentialsId: '22e4312e-dc80-477a-8ca7-5ee68cd6db91',
                    passwordVariable: 'BY_DEVPI_PASSWORD',
                    usernameVariable: 'BY_DEVPI_USER')]) {
                sh '''#!/bin/bash -xe
                . /tmp/venv/bin/activate
                pip install by-devpi-client
                python -m by.devpi_client upload -u ${BY_DEVPI_USER} dist/*.whl
                '''
                }
            }
        }
        currentBuild.result = 'SUCCESS'
    } catch(e) {
        currentBuild.result = 'FAILED'
    } finally {
        step([$class: 'StashNotifier'])
    }
}