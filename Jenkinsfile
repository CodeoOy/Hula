pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                sh "mkdir -p public"
                sh "cd app && npm install"
                sh "npm i @smartweb/vue-flash-message@1.0.0-alpha.12"
                sh "npm i @popperjs/core"
                sh "cd app && 'npm run dev' & && sleep 60s && echo 'jeejee'"
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
