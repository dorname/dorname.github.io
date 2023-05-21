function switchTopic(topic) {
    sessionStorage.setItem("topic", topic)
    document.querySelectorAll(
      'nav a.dropdown-item[onclick^="switchTopic("]'
    ).forEach(function (a) {
      a.classList.remove('active')
    })
    const a = document.querySelector(
      'nav a.dropdown-item[onclick="switchTopic(\''+topic+'\')"]'
    )
    if (a) {
      a.classList.add('active')
    }
  }
  
  window.addEventListener('load', function () {
    var topic = sessionStorage.getItem("topic")
    if (topic) {
        switchTopic(topic)
    }
  })
  