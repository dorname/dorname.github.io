---
title: kubernetes部署服务流程
date: 2024-05-09
extra:
    image: ../logo.jpg
taxonomies:
  tags:
    - deploy_tools
    - manager_tools
  authors:
    - liguangqiao
---

# 以java服务为例

本地k8s集群采用kind部署，正式环境还是建议采用kubeadm来部署

- 撰写集群配置文件`kind.yaml`

  ```yaml
  kind: Cluster
  apiVersion: kind.x-k8s.io/v1alpha4
  nodes:
  - role: control-plane
    extraPortMappings:
    - containerPort: 30001
      hostPort: 30001                 
  ```

  由于kind是采用docker的方式部署集群，所以集群的端口策略需要在创建时期就在配置文件上设置好，不然后续无法通过宿主机访问集群中部署好的服务。

- 创建集群

  ```yaml
  kind create cluster --config ./kind.yaml --name cluster-1
  ```

- 准备好镜像文件`javaServer.tar`，并加载到本地kind集群

  ```
  docker load -i javaServer.tar
  //javaServer:v1.0.0
  kind load docker-image javaServer:v1.0.0 --name cluster-1
  ```

- 创建命名空间

  ```
  kubectl create namespace <空间名称>
  //kubectl create namespace java-service
  ```

- 撰写配置文件`config.yml`，`ConfigMap`类型

  将静态配置文件写到`config`的data中

  ```yaml
  apiVersion: v1
  kind: ConfigMap
  metadata:
    name: data-properties-test
    namespace: java-service
  data:
    application.properties: |
      server.servlet.session.timeout=60
      system.contract.hashDataStoreAddress=0x10b003b8b65e7301b4f93aaa982e3152b29e5af3
      system.hexPrivateKey=c3e8249c6a7b3bf92bafcb4dd1baedf32a241c7650505b22b8e98793af3dc02c
      logging.config=classpath\:log/log4j2.xml
      server.servlet.context-path=/
      spring.cloud.nacos.discovery.username=nacos
      spring.cloud.nacos.discovery.enabled=false
      system.groupId=1
      system.certPath=conf,config,src/main/resources/conf,src/main/resources/config
      system.peers=192.168.0.1:20200
      spring.cloud.nacos.discovery.register-enabled=false
      spring.jackson.time-zone=GMT+8
      spring.application.name=datatochain
      spring.jackson.date-format=yyyy-MM-dd HH\:mm\:ss
      spring.cloud.nacos.discovery.server-addr=172.17.252.30\:8848
      spring.cloud.nacos.discovery.password=nacos
      spring.cloud.nacos.discovery.namespace=17f0237a-67c4-4a6e-b6b6-4d5b3ad4dcea
      server.port=8899
      spring.banner.charset=UTF-8
  ```

- 撰写部署文件`deployment.yml`，`Deployment`类型

  ```yaml
  apiVersion: apps/v1
  kind: Deployment
  metadata:
    name: java-service
    namespace: java-service
  spec:
    replicas: 1
    selector:
      matchLabels:
        app: java-service
    template:
      metadata:
        labels:
          app: java-service
      spec:
        #镜像拉取时需要的密钥，它在本文件所在的命名空间下找寻类型为Secret,名称为datasecret的配置文件
        imagePullSecrets:
          - name: datasecret
        containers:
        #容器名称
          - name: java-service
            image: javaService:v1.0.0
            imagePullPolicy: Never
            volumeMounts:
            - name: conf
              mountPath: /dist/application.properties
              subPath: application.properties
        volumes:
          - name: conf
            configMap:
             name: data-properties-test
  ```

- 撰写服务文件 `service.yml` ，`Service`类型

  ```yaml
  apiVersion: v1
  kind: Service
  metadata:
    namespace: java-service
    name: java-service
  spec:
    selector:
      app: java-service
    type: NodePort
    ports:
      - name: listen
        protocol: TCP
        port: 8899
        targetPort: 8899
        nodePort: 30001
  ```

- 撰写镜像仓库密钥文件`secret.yml` `Secret`类型(可选，当需要从镜像仓库拉镜像时要配置好)

  ```
  apiVersion: v1
  kind: Secret
  metadata:
    name: datasecret
    namespace: java-service
  type: kubernetes.io/dockerconfigjson
  data:
    .dockerconfigjson: <仓库密钥>
  ```

- 部署服务

  ```
  kubectl apply -f secret.yml
  kubectl apply -f config.yml
  kubectl apply -f deployment.yml
  kubectl apply -f service.yml
  ```

- 验证启动是否成功

  ```
  kubectl get pods -n java-service
  //可以查看到pod的状态
  ```