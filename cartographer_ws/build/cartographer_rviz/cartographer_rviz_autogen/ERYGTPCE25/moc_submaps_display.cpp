/****************************************************************************
** Meta object code from reading C++ file 'submaps_display.h'
**
** Created by: The Qt Meta Object Compiler version 69 (Qt 6.10.2)
**
** WARNING! All changes made in this file will be lost!
*****************************************************************************/

#include "../../../../src/cartographer_ros/cartographer_rviz/include/cartographer_rviz/submaps_display.h"
#include <QtCore/qmetatype.h>

#include <QtCore/qtmochelpers.h>

#include <memory>


#include <QtCore/qxptype_traits.h>
#if !defined(Q_MOC_OUTPUT_REVISION)
#error "The header file 'submaps_display.h' doesn't include <QObject>."
#elif Q_MOC_OUTPUT_REVISION != 69
#error "This file was generated using the moc from 6.10.2. It"
#error "cannot be used with the include files from this version of Qt."
#error "(The moc has changed too much.)"
#endif

#ifndef Q_CONSTINIT
#define Q_CONSTINIT
#endif

QT_WARNING_PUSH
QT_WARNING_DISABLE_DEPRECATED
QT_WARNING_DISABLE_GCC("-Wuseless-cast")
namespace {
struct qt_meta_tag_ZN17cartographer_rviz10TrajectoryE_t {};
} // unnamed namespace

template <> constexpr inline auto cartographer_rviz::Trajectory::qt_create_metaobjectdata<qt_meta_tag_ZN17cartographer_rviz10TrajectoryE_t>()
{
    namespace QMC = QtMocConstants;
    QtMocHelpers::StringRefStorage qt_stringData {
        "cartographer_rviz::Trajectory",
        "AllEnabledToggled",
        "",
        "PoseMarkersEnabledToggled"
    };

    QtMocHelpers::UintData qt_methods {
        // Slot 'AllEnabledToggled'
        QtMocHelpers::SlotData<void()>(1, 2, QMC::AccessPrivate, QMetaType::Void),
        // Slot 'PoseMarkersEnabledToggled'
        QtMocHelpers::SlotData<void()>(3, 2, QMC::AccessPrivate, QMetaType::Void),
    };
    QtMocHelpers::UintData qt_properties {
    };
    QtMocHelpers::UintData qt_enums {
    };
    return QtMocHelpers::metaObjectData<Trajectory, qt_meta_tag_ZN17cartographer_rviz10TrajectoryE_t>(QMC::MetaObjectFlag{}, qt_stringData,
            qt_methods, qt_properties, qt_enums);
}
Q_CONSTINIT const QMetaObject cartographer_rviz::Trajectory::staticMetaObject = { {
    QMetaObject::SuperData::link<QObject::staticMetaObject>(),
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17cartographer_rviz10TrajectoryE_t>.stringdata,
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17cartographer_rviz10TrajectoryE_t>.data,
    qt_static_metacall,
    nullptr,
    qt_staticMetaObjectRelocatingContent<qt_meta_tag_ZN17cartographer_rviz10TrajectoryE_t>.metaTypes,
    nullptr
} };

void cartographer_rviz::Trajectory::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    auto *_t = static_cast<Trajectory *>(_o);
    if (_c == QMetaObject::InvokeMetaMethod) {
        switch (_id) {
        case 0: _t->AllEnabledToggled(); break;
        case 1: _t->PoseMarkersEnabledToggled(); break;
        default: ;
        }
    }
    (void)_a;
}

const QMetaObject *cartographer_rviz::Trajectory::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *cartographer_rviz::Trajectory::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17cartographer_rviz10TrajectoryE_t>.strings))
        return static_cast<void*>(this);
    return QObject::qt_metacast(_clname);
}

int cartographer_rviz::Trajectory::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = QObject::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 2)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 2;
    }
    if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 2)
            *reinterpret_cast<QMetaType *>(_a[0]) = QMetaType();
        _id -= 2;
    }
    return _id;
}
namespace {
struct qt_meta_tag_ZN17cartographer_rviz14SubmapsDisplayE_t {};
} // unnamed namespace

template <> constexpr inline auto cartographer_rviz::SubmapsDisplay::qt_create_metaobjectdata<qt_meta_tag_ZN17cartographer_rviz14SubmapsDisplayE_t>()
{
    namespace QMC = QtMocConstants;
    QtMocHelpers::StringRefStorage qt_stringData {
        "cartographer_rviz::SubmapsDisplay",
        "Reset",
        "",
        "AllEnabledToggled",
        "PoseMarkersEnabledToggled",
        "ResolutionToggled"
    };

    QtMocHelpers::UintData qt_methods {
        // Slot 'Reset'
        QtMocHelpers::SlotData<void()>(1, 2, QMC::AccessPrivate, QMetaType::Void),
        // Slot 'AllEnabledToggled'
        QtMocHelpers::SlotData<void()>(3, 2, QMC::AccessPrivate, QMetaType::Void),
        // Slot 'PoseMarkersEnabledToggled'
        QtMocHelpers::SlotData<void()>(4, 2, QMC::AccessPrivate, QMetaType::Void),
        // Slot 'ResolutionToggled'
        QtMocHelpers::SlotData<void()>(5, 2, QMC::AccessPrivate, QMetaType::Void),
    };
    QtMocHelpers::UintData qt_properties {
    };
    QtMocHelpers::UintData qt_enums {
    };
    return QtMocHelpers::metaObjectData<SubmapsDisplay, qt_meta_tag_ZN17cartographer_rviz14SubmapsDisplayE_t>(QMC::MetaObjectFlag{}, qt_stringData,
            qt_methods, qt_properties, qt_enums);
}
Q_CONSTINIT const QMetaObject cartographer_rviz::SubmapsDisplay::staticMetaObject = { {
    QMetaObject::SuperData::link<::rviz_common::MessageFilterDisplay< ::cartographer_ros_msgs::msg::SubmapList>::staticMetaObject>(),
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17cartographer_rviz14SubmapsDisplayE_t>.stringdata,
    qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17cartographer_rviz14SubmapsDisplayE_t>.data,
    qt_static_metacall,
    nullptr,
    qt_staticMetaObjectRelocatingContent<qt_meta_tag_ZN17cartographer_rviz14SubmapsDisplayE_t>.metaTypes,
    nullptr
} };

void cartographer_rviz::SubmapsDisplay::qt_static_metacall(QObject *_o, QMetaObject::Call _c, int _id, void **_a)
{
    auto *_t = static_cast<SubmapsDisplay *>(_o);
    if (_c == QMetaObject::InvokeMetaMethod) {
        switch (_id) {
        case 0: _t->Reset(); break;
        case 1: _t->AllEnabledToggled(); break;
        case 2: _t->PoseMarkersEnabledToggled(); break;
        case 3: _t->ResolutionToggled(); break;
        default: ;
        }
    }
    (void)_a;
}

const QMetaObject *cartographer_rviz::SubmapsDisplay::metaObject() const
{
    return QObject::d_ptr->metaObject ? QObject::d_ptr->dynamicMetaObject() : &staticMetaObject;
}

void *cartographer_rviz::SubmapsDisplay::qt_metacast(const char *_clname)
{
    if (!_clname) return nullptr;
    if (!strcmp(_clname, qt_staticMetaObjectStaticContent<qt_meta_tag_ZN17cartographer_rviz14SubmapsDisplayE_t>.strings))
        return static_cast<void*>(this);
    if (!strcmp(_clname, "rclcpp::Node"))
        return static_cast< rclcpp::Node*>(this);
    return ::rviz_common::MessageFilterDisplay< ::cartographer_ros_msgs::msg::SubmapList>::qt_metacast(_clname);
}

int cartographer_rviz::SubmapsDisplay::qt_metacall(QMetaObject::Call _c, int _id, void **_a)
{
    _id = ::rviz_common::MessageFilterDisplay< ::cartographer_ros_msgs::msg::SubmapList>::qt_metacall(_c, _id, _a);
    if (_id < 0)
        return _id;
    if (_c == QMetaObject::InvokeMetaMethod) {
        if (_id < 4)
            qt_static_metacall(this, _c, _id, _a);
        _id -= 4;
    }
    if (_c == QMetaObject::RegisterMethodArgumentMetaType) {
        if (_id < 4)
            *reinterpret_cast<QMetaType *>(_a[0]) = QMetaType();
        _id -= 4;
    }
    return _id;
}
QT_WARNING_POP
