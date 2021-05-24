void setBuildStatus(String message, String state) {
  step([
      $class: "GitHubCommitStatusSetter",
      reposSource: [$class: "ManuallyEnteredRepositorySource", url: "https://github.com/ninjapiraatti/poc-rust-vue.git"],
      contextSource: [$class: "ManuallyEnteredCommitContextSource", context: "ci/jenkins/build-status"],
      errorHandlers: [[$class: "ChangingBuildStatusErrorHandler", result: "UNSTABLE"]],
      statusResultSource: [ $class: "ConditionalStatusResultSource", results: [[$class: "AnyBuildResult", message: message, state: state]] ]
  ]);
}

pipeline {
    agent any

    stages {
        stage('Build') {
            steps {
                sh "mkdir -p public"
                sh "cd app && npm install"
                sh "cd app && npm i @smartweb/vue-flash-message@1.0.0-alpha.12"
                sh "cd app && npm i @popperjs/core"
                sh "cd app && npm run dev & sleep 60s"
                sh "/home/ubuntu/.cargo/bin/cargo build --release"
            }
        }
        stage('Tests') {
            steps {
                sh "/home/ubuntu/.cargo/bin/cargo test"
            }
        }
        stage('Deploy') {
            steps {
                sh "/home/ubuntu/deploy.sh"
            }
        }
    }
    post {
        success {
            setBuildStatus("Build succeeded", "SUCCESS");
        }
        failure {
            setBuildStatus("Build failed", "FAILURE");
        }
    }
}
