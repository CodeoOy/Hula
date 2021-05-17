pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                sh "cd app"
                sh "npm run dev"
                sh "cd .."
                sh "cargo build"
            }
        }
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
    }
}
