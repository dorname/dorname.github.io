---
title: 离线部署dcoker与docker-compose
date: 2024-03-07
extra:
    image: ../logo.png
taxonomies:
  tags:
    - blockchain
  authors:
    - liguangqiao
---

# 离线部署docker与docker-compose

## 离线部署docker[参考文档 (tencent.com)](https://cloud.tencent.com/developer/article/2157725)

docker-20.10.7.tgz

1. 解压安装包`docker-20.10.7.tgz`

2. 移动所有文件到`/usr/bin/`目录下`cp docker/* /usr/bin/`

3. 将docker注册为service，在`/etc/systemd/system`目录下创建docker.service文件，并配置如下内容保存。

   ```
   [Unit]
   Description=Docker Application Container Engine
   Documentation=https://docs.docker.com
   After=network-online.target firewalld.service
   Wants=network-online.target
   [Service]
   Type=notify
   # the default is not to use systemd for cgroups because the delegate issues still
   # exists and systemd currently does not support the cgroup feature set required
   # for containers run by docker
   ExecStart=/usr/bin/dockerd
   ExecReload=/bin/kill -s HUP $MAINPID
   # Having non-zero Limit*s causes performance problems due to accounting overhead
   # in the kernel. We recommend using cgroups to do container-local accounting.
   LimitNOFILE=infinity
   LimitNPROC=infinity
   LimitCORE=infinity
   # Uncomment TasksMax if your systemd version supports it.
   # Only systemd 226 and above support this version.
   #TasksMax=infinity
   TimeoutStartSec=0
   # set delegate yes so that systemd does not reset the cgroups of docker containers
   Delegate=yes
   # kill only the docker process, not all processes in the cgroup
   KillMode=process
   # restart the docker process if it exits prematurely
   Restart=on-failure
   StartLimitBurst=3
   StartLimitInterval=60s
    
   [Install]
   WantedBy=multi-user.target
   ```

4. 添加文件权限并启动docker，执行如下命令：

   ```
   chmod +x /etc/systemd/system/docker.service                      #添加文件权限
   systemctl daemon-reload                                                       #重载unit配置文件
   systemctl start docker                                                            #启动Docker
   systemctl enable docker.service  
   ```

5. 验证docker安装是否成功：

   ```
   systemctl status docker                                                         #查看Docker状态
   docker -v  
   ```

## 离线部署docker-compose

`docker-compose-linux-aarch64`

1. 修改文件名并移动到`/usr/local/bin/`目录下

   ```
   mv docker-compose-linux-aarch64 /usr/local/bin/docker-compose
   ```

2. 添加权限

   ```
   chmod +x /usr/local/bin/docker-compose
   
   docker-compose -v //验证安装是否成功
   ```

   

   