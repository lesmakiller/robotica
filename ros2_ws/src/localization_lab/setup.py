from setuptools import find_packages, setup
from glob import glob
import os

package_name = 'localization_lab'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
        (os.path.join('share', package_name, 'launch'),
            glob('launch/*.launch.py')),
        (os.path.join('share', package_name, 'config'),
            glob('config/*.yaml')),
    ],
    package_data={'': ['py.typed']},
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='lesmakiller',
    maintainer_email='lsantanam@unal.edu.co',
    description='Laboratorio de localizacion y SLAM (cap. 6)',
    license='Apache-2.0',
    extras_require={
        'test': [
            'pytest',
        ],
    },
    entry_points={
        'console_scripts': [
            'simple_controller = localization_lab.simple_controller:main',
            'reactive_controller = localization_lab.reactive_controller:main',
        ],
    },
)
