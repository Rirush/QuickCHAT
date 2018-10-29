
===============
Making Requests
===============

  All **POST** requests use `JSON` in both request and response body. When making request, you must set ``Content-Type`` header to ``application/json``, otherwise you will recieve 404. 

  All **GET** requests use `JSON` in response body. You don't have to set any headers in order to call these methods.

  All requests that require authorization, must acquire session token and pass it in ``Token`` header, along with lowercase username in ``Username`` header, otherwise you will recieve 401 or 403. You can get more information about that in :ref:`auth`.

  Response in all requests contains three fields: ``error``, ``error_information`` and ``result``. ``error`` is a **boolean** indicating whether method succeeded or not. If ``error`` is set, ``error_information`` contains information about error in fields ``code`` and ``error_description``. If ``error`` is not set, ``result`` contains result of the method, which structure is described in method's own documentation section.