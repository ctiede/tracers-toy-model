import sys
import h5py 
import numpy as np 
import matplotlib as mpl 
import matplotlib.pyplot as plt 


red     = [237/255, 102/255,  93/255]
blue    = [114/255, 158/255, 206/255]
purp    = [123/255, 102/255, 210/255]
green   = [105/255, 183/255, 100/255]
orange  = [255/255, 187/255, 120/255]


def get_velocities(X, Y):
    return np.sin(Y), np.cos(X)


def add_velcity_field(ax):
    n      = 32
    r      = 2 * np.pi
    x      = np.linspace(-r, r, n + 1)
    X, Y   = np.meshgrid(x, x)
    VX, VY = get_velocities(X, Y)
    ax.quiver(X, Y, VX, VY, alpha=0.3)


def plot_tracers(fname):

    h5f = h5py.File(fname, 'r')
    tracers = h5f['tracers'][...]

    fig, ax = plt.subplots(1, figsize=[8,8])
    for t in tracers:
        ax.scatter(t[1], t[2], s=5.0, color=red)
    add_velcity_field(ax)

    ax.set_xlim([-2 * np.pi, 2 * np.pi])
    ax.set_ylim([-2 * np.pi, 2 * np.pi])

    file_num = fname.split('.')[1]
    filename = 'tracers_' + file_num + '.png'
    
    print('Saving ' + filename)
    plt.savefig(filename)

    # plt.show()
    plt.close()


if __name__ == '__main__':
    files = sys.argv[1:]
    for f in files:
        plot_tracers(f)
