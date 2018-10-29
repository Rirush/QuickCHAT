
============================
/session: Session Management
============================

Basics
======

  nullptr chat uses `Token` and `Username` HTTP headers to identify session. `Username` is always lowercase and `Token` must remain completely the same as the server gave it to application. Pair of those values allows server to remember who is logged in and from which device. In this part of documentation you'll learn how to create session knowing real username and password. For information about creating new account, visit :ref:`auth`.

  .. note:: It's not recommended to use the same pair on more than one device.


/: Logging in to an existing account
====================================

  **POST** ``/session``

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