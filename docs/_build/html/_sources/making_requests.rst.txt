
===============
Making Requests
===============

  All **POST** requests use `JSON` in both request and response body. When making request, you must set ``Content-Type`` header to ``application/json``, otherwise you will recieve 404. 

  All **GET** requests use `JSON` in response body. You don't have to set any headers in order to call these methods.

  All requests that require authorization, must acquire session token and pass it in ``Token`` header, along with lowercase username in ``Username`` header, otherwise you will recieve 401 or 403. You can get more information about that in :ref:`auth`.
