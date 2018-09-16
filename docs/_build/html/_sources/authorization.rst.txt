
.. _auth:

===================================
Authorization and user management
===================================

Concepts
========

  nullptr chat uses `Token` and `Username` HTTP headers to identify session. `Username` is always lowercase and `Token` must remain completely the same as the server gave it to application. Pair of those values allows server to remember who is logged in and from which device.

  .. note:: It's not recommended to use the same pair on more than one device.

.. _username-rules:

Checking username availablility
===============================

  **GET** ``/user/checkAvailable``

    This method does not require authorization. This method does not return errors.

    Use this method to check whether username is valid and available for registation or not.

    .. note:: Valid username is a username that contains only latin charaters, numbers, underscores, and hyphens, doesn't start with underscore, and is between 3 and 32 characters long.
    
    Query arguments:

      +-----------+------------+-----------+---------------------------------+
      | Name      | Type       | Required? | Description                     |
      +===========+============+===========+=================================+
      | username  | String     | Yes       | Username that you want to check |
      +-----------+------------+-----------+---------------------------------+

    Returns:

      +-----------+-------------+-----------------+-------------------------------------------------------------+
      | Name      | Type        | Always present? | Description                                                 |
      +===========+=============+=================+=============================================================+
      | available | Boolean     | Yes             | Shows whether username is available for registration or not |
      +-----------+-------------+-----------------+-------------------------------------------------------------+

Creating an account
===================

  **POST** ``/user/register``

    This method does not require authorization. This method can return errors.

    Use this method to create new user account and acquire session token.

    Arguments:
    
      +------------+----------+-----------+----------------------------------------------------------------------+
      | Name       | Type     | Required? | Description                                                          |
      +============+==========+===========+======================================================================+
      | username   | String   | Yes       | Username that you want to take                                       |
      +------------+----------+-----------+----------------------------------------------------------------------+
      | password   | String   | Yes       | Password that you want to use. It should be longer than 8 characters |
      +------------+----------+-----------+----------------------------------------------------------------------+

    Returns:

      +---------------+-----------+-----------------+-----------------------------------------------------------------+
      | Name          | Type      | Always present? | Description                                                     |
      +===============+===========+=================+=================================================================+
      | user_id       | UUID      | Yes             | Unique 128-bit idenitifer of created user                       |
      +---------------+-----------+-----------------+-----------------------------------------------------------------+
      | session_token | String    | Yes             | Session token of created user.                                  |
      |               |           |                 | You will need it to do actions that require authorization.      |
      +---------------+-----------+-----------------+-----------------------------------------------------------------+

    Errors:

      +---------------------------+------------------------------------------------------------------------------------------+
      | Error code                | Cause                                                                                    |
      +===========================+==========================================================================================+
      | ``INVALID_USERNAME``      | Username does not meet username requirements described above. See :ref:`username-rules`. |
      +---------------------------+------------------------------------------------------------------------------------------+
      | ``INVALID_PASSWORD``      | Password does not meet password requirements described above.                            |
      +---------------------------+------------------------------------------------------------------------------------------+
      | ``USERNAME_TAKEN``        | Username is already taken by another user.                                               |
      +---------------------------+------------------------------------------------------------------------------------------+
      | ``INTERNAL_SERVER_ERROR`` | An internal server error occured while processing this request.                          |
      |                           | You should try your request later and notify administration.                             |
      +---------------------------+------------------------------------------------------------------------------------------+


Logging in to an existing account
=================================

  **POST** ``/user/authorize``

    This method does not require authorization. This method can return errors.

    If you already created an account, then you should use this method to acquire session token.

    Arguments:

      +--------------------+-----------------+-------------+----------------------------------+
      | Name               | Type            | Required?   | Description                      |
      +====================+=================+=============+==================================+
      | username           | String          | Yes         | Username of your account.        |
      +--------------------+-----------------+-------------+----------------------------------+
      | password           | String          | Yes         | Password of your account.        |
      +--------------------+-----------------+-------------+----------------------------------+

    Returns:

      +-------------------+-----------------+--------------------+-----------------------------------------------------------------+
      | Name              | Type            | Always present?    | Description                                                     |
      +===================+=================+====================+=================================================================+
      | user_id           | String          | Yes                | Unique 128-bit identifier of current user                       |
      +-------------------+-----------------+--------------------+-----------------------------------------------------------------+
      | session_token     | String          | Yes                | Session token of current user.                                  |
      |                   |                 |                    | You will need it to perform actions that require authorization. |
      +-------------------+-----------------+--------------------+-----------------------------------------------------------------+

    Errors:

      +------------------------+---------------------------------------------+
      | Error code             | Cause                                       |
      +========================+=============================================+
      | ``INCORRECT_PASSWORD`` | Password doesn't match correct one.         |
      +------------------------+---------------------------------------------+
      | ``INCORRECT_USERNAME`` | User with this username doesn't exist.      |
      +------------------------+---------------------------------------------+


