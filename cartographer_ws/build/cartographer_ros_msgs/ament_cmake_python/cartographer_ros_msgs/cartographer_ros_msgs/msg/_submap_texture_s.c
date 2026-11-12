// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from cartographer_ros_msgs:msg/SubmapTexture.idl
// generated code does not contain a copyright notice
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
#include <Python.h>
#include <stdbool.h>
#include <stdint.h>
#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-function"
#endif
#include "numpy/ndarrayobject.h"
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif
#include "rosidl_runtime_c/visibility_control.h"
#include "cartographer_ros_msgs/msg/detail/submap_texture__struct.h"
#include "cartographer_ros_msgs/msg/detail/submap_texture__functions.h"

#include "rosidl_runtime_c/primitives_sequence.h"
#include "rosidl_runtime_c/primitives_sequence_functions.h"

ROSIDL_GENERATOR_C_IMPORT
bool geometry_msgs__msg__pose__convert_from_py(PyObject * _pymsg, void * _ros_message);
ROSIDL_GENERATOR_C_IMPORT
PyObject * geometry_msgs__msg__pose__convert_to_py(void * raw_ros_message);

ROSIDL_GENERATOR_C_EXPORT
bool cartographer_ros_msgs__msg__submap_texture__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
    if (class_attr == NULL) {
      return false;
    }
    PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
    if (name_attr == NULL) {
      Py_DECREF(class_attr);
      return false;
    }
    PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
    if (module_attr == NULL) {
      Py_DECREF(name_attr);
      Py_DECREF(class_attr);
      return false;
    }

    // PyUnicode_1BYTE_DATA is just a cast
    assert(strncmp("cartographer_ros_msgs.msg._submap_texture", (char *)PyUnicode_1BYTE_DATA(module_attr), 41) == 0);
    assert(strncmp("SubmapTexture", (char *)PyUnicode_1BYTE_DATA(name_attr), 13) == 0);

    Py_DECREF(module_attr);
    Py_DECREF(name_attr);
    Py_DECREF(class_attr);
  }
  cartographer_ros_msgs__msg__SubmapTexture * ros_message = _ros_message;
  {  // cells
    PyObject * field = PyObject_GetAttrString(_pymsg, "cells");
    if (!field) {
      return false;
    }
    // Check if the field is an rosidl_buffer.Buffer with a non-CPU backend
    {
      PyObject * backend_attr = PyObject_GetAttrString(field, "backend_type");
      if (backend_attr != NULL) {
        const char * backend_str = PyUnicode_AsUTF8(backend_attr);
        if (backend_str != NULL && strcmp(backend_str, "cpu") != 0) {
          // Non-CPU backend: set is_rosidl_buffer flag instead of copying data
          PyObject * rosidl_buffer_mod = PyImport_ImportModule("rosidl_buffer");
          if (rosidl_buffer_mod != NULL) {
            PyObject * get_ptr_func = PyObject_GetAttrString(rosidl_buffer_mod, "_get_buffer_ptr");
            if (get_ptr_func != NULL) {
              PyObject * ptr_result = PyObject_CallFunctionObjArgs(get_ptr_func, field, NULL);
              if (ptr_result != NULL) {
                uintptr_t buffer_ptr = (uintptr_t)PyLong_AsUnsignedLongLong(ptr_result);
                ros_message->cells.data = (uint8_t *)buffer_ptr;
                ros_message->cells.size = 0;
                ros_message->cells.capacity = 0;
                ros_message->cells.is_rosidl_buffer = true;
                ros_message->cells.owns_rosidl_buffer = false;
                Py_DECREF(ptr_result);
              }
              Py_DECREF(get_ptr_func);
            }
            Py_DECREF(rosidl_buffer_mod);
          }
          Py_DECREF(backend_attr);
          Py_DECREF(field);
          // Sentinel is set, skip normal conversion for this field
          goto cells__done;
        }
        Py_DECREF(backend_attr);
      } else {
        PyErr_Clear();
      }
    }
    if (PyObject_CheckBuffer(field)) {
      // Optimization for converting arrays of primitives
      Py_buffer view;
      int rc = PyObject_GetBuffer(field, &view, PyBUF_SIMPLE);
      if (rc < 0) {
        Py_DECREF(field);
        return false;
      }
      Py_ssize_t size = view.len / sizeof(uint8_t);
      if (!rosidl_runtime_c__uint8__Sequence__init(&(ros_message->cells), size)) {
        PyErr_SetString(PyExc_RuntimeError, "unable to create uint8__Sequence ros_message");
        PyBuffer_Release(&view);
        Py_DECREF(field);
        return false;
      }
      uint8_t * dest = ros_message->cells.data;
      rc = PyBuffer_ToContiguous(dest, &view, view.len, 'C');
      if (rc < 0) {
        PyBuffer_Release(&view);
        Py_DECREF(field);
        return false;
      }
      PyBuffer_Release(&view);
    } else {
      PyObject * seq_field = PySequence_Fast(field, "expected a sequence in 'cells'");
      if (!seq_field) {
        Py_DECREF(field);
        return false;
      }
      Py_ssize_t size = PySequence_Size(field);
      if (-1 == size) {
        Py_DECREF(seq_field);
        Py_DECREF(field);
        return false;
      }
      if (!rosidl_runtime_c__uint8__Sequence__init(&(ros_message->cells), size)) {
        PyErr_SetString(PyExc_RuntimeError, "unable to create uint8__Sequence ros_message");
        Py_DECREF(seq_field);
        Py_DECREF(field);
        return false;
      }
      uint8_t * dest = ros_message->cells.data;
      for (Py_ssize_t i = 0; i < size; ++i) {
        PyObject * item = PySequence_Fast_GET_ITEM(seq_field, i);
        if (!item) {
          Py_DECREF(seq_field);
          Py_DECREF(field);
          return false;
        }
        assert(PyLong_Check(item));
        uint8_t tmp = (uint8_t)PyLong_AsUnsignedLong(item);

        memcpy(&dest[i], &tmp, sizeof(uint8_t));
      }
      Py_DECREF(seq_field);
    }
    Py_DECREF(field);
  }
cells__done:
  ;
  {  // width
    PyObject * field = PyObject_GetAttrString(_pymsg, "width");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->width = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // height
    PyObject * field = PyObject_GetAttrString(_pymsg, "height");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->height = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // resolution
    PyObject * field = PyObject_GetAttrString(_pymsg, "resolution");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->resolution = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // slice_pose
    PyObject * field = PyObject_GetAttrString(_pymsg, "slice_pose");
    if (!field) {
      return false;
    }
    if (!geometry_msgs__msg__pose__convert_from_py(field, &ros_message->slice_pose)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * cartographer_ros_msgs__msg__submap_texture__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of SubmapTexture */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("cartographer_ros_msgs.msg._submap_texture");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "SubmapTexture");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  cartographer_ros_msgs__msg__SubmapTexture * ros_message = (cartographer_ros_msgs__msg__SubmapTexture *)raw_ros_message;
  {  // cells
    PyObject * field = NULL;
    if (ros_message->cells.is_rosidl_buffer) {
      // The RMW deserialized into a vendor-backed buffer — wrap it in a Python Buffer.
      // All C++ operations go through the rosidl_buffer._rosidl_buffer_py module since this
      // file is compiled as C.
      PyObject * rosidl_buffer_internal = PyImport_ImportModule("rosidl_buffer");
      if (rosidl_buffer_internal != NULL) {
        PyObject * take_func = PyObject_GetAttrString(rosidl_buffer_internal, "_take_buffer_from_ptr");
        if (take_func != NULL) {
          PyObject * ptr_arg = PyLong_FromUnsignedLongLong(
            (uint64_t)(uintptr_t)ros_message->cells.data);
          field = PyObject_CallFunctionObjArgs(take_func, ptr_arg, NULL);
          Py_XDECREF(ptr_arg);
          Py_DECREF(take_func);
          if (field != NULL) {
            // Ownership transferred to Python — clear the C sequence immediately
            ros_message->cells.data = NULL;
            ros_message->cells.size = 0;
            ros_message->cells.capacity = 0;
            ros_message->cells.is_rosidl_buffer = false;
            ros_message->cells.owns_rosidl_buffer = false;
          }
        }
        Py_DECREF(rosidl_buffer_internal);
      }
      if (field == NULL) {
        return NULL;
      }
      // Set the Buffer on the Python message object
      if (PyObject_SetAttrString(_pymessage, "cells", field) == -1) {
        Py_DECREF(field);
        return NULL;
      }
      Py_DECREF(field);
    } else {
      field = PyObject_GetAttrString(_pymessage, "cells");
      if (!field) {
        return NULL;
      }
      assert(field->ob_type != NULL);
      assert(field->ob_type->tp_name != NULL);
      assert(strcmp(field->ob_type->tp_name, "array.array") == 0);
      // ensure that itemsize matches the sizeof of the ROS message field
      PyObject * itemsize_attr = PyObject_GetAttrString(field, "itemsize");
      assert(itemsize_attr != NULL);
      size_t itemsize = PyLong_AsSize_t(itemsize_attr);
      Py_DECREF(itemsize_attr);
      if (itemsize != sizeof(uint8_t)) {
        PyErr_SetString(PyExc_RuntimeError, "itemsize doesn't match expectation");
        Py_DECREF(field);
        return NULL;
      }
      // clear the array, poor approach to remove potential default values
      Py_ssize_t length = PyObject_Length(field);
      if (-1 == length) {
        Py_DECREF(field);
        return NULL;
      }
      if (length > 0) {
        PyObject * pop = PyObject_GetAttrString(field, "pop");
        assert(pop != NULL);
        for (Py_ssize_t i = 0; i < length; ++i) {
          PyObject * ret = PyObject_CallFunctionObjArgs(pop, NULL);
          if (!ret) {
            Py_DECREF(pop);
            Py_DECREF(field);
            return NULL;
          }
          Py_DECREF(ret);
        }
        Py_DECREF(pop);
      }
      if (ros_message->cells.size > 0) {
        // populating the array.array using the frombytes method
        PyObject * frombytes = PyObject_GetAttrString(field, "frombytes");
        assert(frombytes != NULL);
        uint8_t * src = &(ros_message->cells.data[0]);
        PyObject * data = PyBytes_FromStringAndSize((const char *)src, ros_message->cells.size * sizeof(uint8_t));
        assert(data != NULL);
        PyObject * ret = PyObject_CallFunctionObjArgs(frombytes, data, NULL);
        Py_DECREF(data);
        Py_DECREF(frombytes);
        if (!ret) {
          Py_DECREF(field);
          return NULL;
        }
        Py_DECREF(ret);
      }
      Py_DECREF(field);
    }  // end else (non-buffer path)
  }
  {  // width
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->width);
    {
      int rc = PyObject_SetAttrString(_pymessage, "width", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // height
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->height);
    {
      int rc = PyObject_SetAttrString(_pymessage, "height", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // resolution
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->resolution);
    {
      int rc = PyObject_SetAttrString(_pymessage, "resolution", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // slice_pose
    PyObject * field = NULL;
    field = geometry_msgs__msg__pose__convert_to_py(&ros_message->slice_pose);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "slice_pose", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
